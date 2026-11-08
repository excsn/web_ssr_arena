//! An HTTP load client whose every worker sends the next request the moment the last response's body is read.
//!
//! The `hyper` client gives each worker one HTTP/1.1 connection of its own and opens a new one only when that one fails. The `fibre` clients send through one fibre_http pool sized to the concurrency: `fibre` from tokio tasks, `fibre-sync` from one blocking thread per worker and `fibre-h2c` over HTTP/2 without TLS.

use std::time::{Duration, Instant};

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::client::conn::http1::{SendRequest, handshake};
use hyper::{Request, Uri, header};
use hyper_util::rt::TokioIo;
use serde_json::{Value, json};
use tokio::net::TcpStream;

const ERRORS_KEPT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Client {
  Hyper,
  Fibre,
  FibreSync,
  FibreH2c,
}

impl Client {
  pub fn parse(name: &str) -> Result<Client, String> {
    match name {
      "hyper" => Ok(Client::Hyper),
      "fibre" => Ok(Client::Fibre),
      "fibre-sync" => Ok(Client::FibreSync),
      "fibre-h2c" => Ok(Client::FibreH2c),
      other => Err(format!("unknown client {other}: hyper, fibre, fibre-sync or fibre-h2c")),
    }
  }

  /// The client named by `LOAD_CLIENT`, hyper when unset.
  pub fn from_env() -> Result<Client, String> {
    match std::env::var("LOAD_CLIENT") {
      Ok(name) => Client::parse(&name),
      Err(_) => Ok(Client::Hyper),
    }
  }

  pub fn name(self) -> &'static str {
    match self {
      Client::Hyper => "hyper",
      Client::Fibre => "fibre",
      Client::FibreSync => "fibre-sync",
      Client::FibreH2c => "fibre-h2c",
    }
  }
}

#[derive(Default)]
struct Tally {
  latencies: Vec<u64>,
  non2xx: u64,
  failed: u64,
  connections: u64,
  errors: Vec<String>,
}

impl Tally {
  fn fail(&mut self, why: String) {
    self.failed += 1;
    if self.errors.len() < ERRORS_KEPT {
      self.errors.push(why);
    }
  }

  fn absorb(&mut self, other: Tally) {
    self.latencies.extend(other.latencies);
    self.non2xx += other.non2xx;
    self.failed += other.failed;
    self.connections += other.connections;
    for why in other.errors {
      if self.errors.len() < ERRORS_KEPT {
        self.errors.push(why);
      }
    }
  }
}

/// What a run measured. `connections` is the TCP connections opened, so a clean run opens exactly its concurrency.
pub struct Run {
  pub done: usize,
  pub rps: f64,
  pub p50_ms: f64,
  pub p95_ms: f64,
  pub p99_ms: f64,
  pub max_ms: f64,
  pub non2xx: u64,
  pub failed: u64,
  pub connections: u64,
  pub wall_s: f64,
  pub errors: Vec<String>,
}

impl Run {
  pub fn json(&self) -> Value {
    json!({
      "done": self.done,
      "rps": self.rps.round(),
      "p50_ms": hundredths(self.p50_ms),
      "p95_ms": hundredths(self.p95_ms),
      "p99_ms": hundredths(self.p99_ms),
      "max_ms": hundredths(self.max_ms),
      "non2xx": self.non2xx,
      "failed": self.failed,
      "connections": self.connections,
      "wall_s": (self.wall_s * 10.0).round() / 10.0,
    })
  }
}

fn hundredths(ms: f64) -> f64 {
  (ms * 100.0).round() / 100.0
}

struct Target {
  authority: String,
  path: String,
}

fn target(url: &str) -> Result<Target, String> {
  let uri: Uri = url.parse().map_err(|e| format!("{url}: {e}"))?;
  let authority = uri.authority().ok_or_else(|| format!("{url}: no host and port"))?.to_string();
  let path = uri.path_and_query().map(|p| p.as_str().to_owned()).unwrap_or_else(|| "/".to_owned());
  Ok(Target { authority, path })
}

async fn connect(authority: &str) -> Result<SendRequest<Empty<Bytes>>, String> {
  let stream = TcpStream::connect(authority).await.map_err(|e| format!("connect: {e}"))?;
  stream.set_nodelay(true).map_err(|e| format!("nodelay: {e}"))?;
  let (sender, connection) = handshake(TokioIo::new(stream)).await.map_err(|e| format!("handshake: {e}"))?;
  tokio::spawn(connection);
  Ok(sender)
}

fn request(target: &Target) -> Request<Empty<Bytes>> {
  Request::get(target.path.as_str()).header(header::HOST, target.authority.as_str()).body(Empty::new()).expect("a request")
}

async fn worker(target: &Target, deadline: Instant) -> Tally {
  let mut tally = Tally { latencies: Vec::with_capacity(1 << 16), ..Tally::default() };
  let mut sender: Option<SendRequest<Empty<Bytes>>> = None;
  while Instant::now() < deadline {
    let held = match sender.as_mut() {
      Some(held) => held,
      None => match connect(&target.authority).await {
        Ok(fresh) => {
          tally.connections += 1;
          sender.insert(fresh)
        }
        Err(why) => {
          tally.fail(why);
          continue;
        }
      },
    };
    if let Err(e) = held.ready().await {
      sender = None;
      tally.fail(format!("ready: {e}"));
      continue;
    }
    let started = Instant::now();
    let response = match held.send_request(request(target)).await {
      Ok(response) => response,
      Err(e) => {
        sender = None;
        tally.fail(format!("send: {e}"));
        continue;
      }
    };
    let ok = response.status().is_success();
    match response.into_body().collect().await {
      Ok(_) => {
        if !ok {
          tally.non2xx += 1;
        }
        tally.latencies.push(started.elapsed().as_micros() as u64);
      }
      Err(e) => {
        sender = None;
        tally.fail(format!("body: {e}"));
      }
    }
  }
  tally
}

fn fibre_pool(concurrency: usize) -> fibre_http::Pool {
  fibre_http::Pool::builder().max_per_host(concurrency).build()
}

fn record(tally: &mut Tally, started: Instant, result: Result<(bool, usize), fibre_http::Error>) {
  match result {
    Ok((ok, _)) => {
      if !ok {
        tally.non2xx += 1;
      }
      tally.latencies.push(started.elapsed().as_micros() as u64);
    }
    Err(e) => tally.fail(e.to_string()),
  }
}

async fn fibre_worker(client: fibre_http::AsyncClient, url: std::sync::Arc<str>, deadline: Instant) -> Tally {
  let mut tally = Tally { latencies: Vec::with_capacity(1 << 16), ..Tally::default() };
  while Instant::now() < deadline {
    let started = Instant::now();
    let result = async {
      let response = client.get(&*url).send().await?;
      let ok = response.status().is_success();
      Ok((ok, response.bytes().await?.len()))
    }
    .await;
    record(&mut tally, started, result);
  }
  tally
}

fn fibre_sync_worker(client: fibre_http::SyncClient, url: &str, deadline: Instant) -> Tally {
  let mut tally = Tally { latencies: Vec::with_capacity(1 << 16), ..Tally::default() };
  while Instant::now() < deadline {
    let started = Instant::now();
    let result = client.get(url).send().and_then(|response| {
      let ok = response.status().is_success();
      Ok((ok, response.bytes()?.len()))
    });
    record(&mut tally, started, result);
  }
  tally
}

/// `concurrency` hyper workers against `url` for `seconds`.
pub async fn run(url: &str, concurrency: usize, seconds: u64) -> Result<Run, String> {
  run_with(Client::Hyper, url, concurrency, seconds).await
}

/// `concurrency` workers of `client` against `url` for `seconds`.
pub async fn run_with(client: Client, url: &str, concurrency: usize, seconds: u64) -> Result<Run, String> {
  let deadline = Instant::now() + Duration::from_secs(seconds);
  let start = Instant::now();
  let mut all = Tally::default();
  match client {
    Client::Hyper => {
      let target = std::sync::Arc::new(target(url)?);
      let workers: Vec<_> = (0..concurrency)
        .map(|_| {
          let target = target.clone();
          tokio::spawn(async move { worker(&target, deadline).await })
        })
        .collect();
      for worker in workers {
        all.absorb(worker.await.map_err(|e| format!("a worker: {e}"))?);
      }
    }
    Client::Fibre | Client::FibreH2c => {
      let pool = fibre_pool(concurrency);
      let mut builder = fibre_http::AsyncClient::builder().pool(&pool);
      if client == Client::FibreH2c {
        builder = builder.http2_prior_knowledge();
      }
      let fibre = builder.build_async().map_err(|e| e.to_string())?;
      let url: std::sync::Arc<str> = url.into();
      let workers: Vec<_> = (0..concurrency).map(|_| tokio::spawn(fibre_worker(fibre.clone(), url.clone(), deadline))).collect();
      for worker in workers {
        all.absorb(worker.await.map_err(|e| format!("a worker: {e}"))?);
      }
      all.connections = pool.connections_opened() as u64;
    }
    Client::FibreSync => {
      let pool = fibre_pool(concurrency);
      let fibre = fibre_http::SyncClient::builder().pool(&pool).build_sync().map_err(|e| e.to_string())?;
      let url = url.to_owned();
      let tallies = tokio::task::spawn_blocking(move || {
        std::thread::scope(|scope| {
          let threads: Vec<_> = (0..concurrency).map(|_| scope.spawn(|| fibre_sync_worker(fibre.clone(), &url, deadline))).collect();
          threads.into_iter().map(|t| t.join().expect("a worker")).collect::<Vec<_>>()
        })
      })
      .await
      .map_err(|e| format!("the workers: {e}"))?;
      for tally in tallies {
        all.absorb(tally);
      }
      all.connections = pool.connections_opened() as u64;
    }
  }
  let wall_s = start.elapsed().as_secs_f64();
  all.latencies.sort_unstable();
  let l = &all.latencies;
  let pct = |p: f64| if l.is_empty() { 0.0 } else { l[((l.len() as f64 - 1.0) * p) as usize] as f64 / 1000.0 };
  Ok(Run {
    done: l.len(),
    rps: l.len() as f64 / wall_s,
    p50_ms: pct(0.50),
    p95_ms: pct(0.95),
    p99_ms: pct(0.99),
    max_ms: pct(1.0),
    non2xx: all.non2xx,
    failed: all.failed,
    connections: all.connections,
    wall_s,
    errors: all.errors,
  })
}

/// One request on a connection of its own, for its status.
pub async fn status(url: &str) -> Result<u16, String> {
  let target = target(url)?;
  let mut sender = connect(&target.authority).await?;
  let response = sender.send_request(request(&target)).await.map_err(|e| format!("send: {e}"))?;
  let status = response.status().as_u16();
  response.into_body().collect().await.map_err(|e| format!("body: {e}"))?;
  Ok(status)
}
