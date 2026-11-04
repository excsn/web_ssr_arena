mod apps;
mod builds;
mod check;
mod fetch;
mod machine;
mod pages;
mod perf;
mod report;
mod server;
mod sizes;

use std::path::PathBuf;

use serde_json::{Value, json};

const USAGE: &str = "usage: web_ssr_arena <command> [flags]
  check                 every page of every app answers what it must, nothing measured
  build                 time a cold build and warm rebuilds of each app, then leave each built
  size                  the deploy tree, server binary, runtime and the bytes each page loads
  run                   load each page at each concurrency level
  report                rewrite docs/ from docs/raw/, nothing measured
  list                  print the apps, pages and default cells

flags:
  --app a,b             apps to include (default: all)
  --page a,b            pages to include, run only (default: all)
  --levels 1,8,32       concurrency levels, run only (default 1,8,32,64,128,256)
  --secs S              seconds per cell, run only (default 10)
  --warmup S            seconds of load on each page before its cells, run only (default 3)
  --reps N              repetitions, interleaved across apps (default: run 3, build 3)
  --no-cold             build only: skip the cold build
  --machine LABEL       label the run (default: the CPU and core count)
  --any-power           measure outside high power mode
  --resume              run only: continue the run in runs/perf.jsonl, measuring only the cells it lacks

  import-log <log>      write runs/perf.jsonl from the printed output of a run that was stopped";

struct Args {
  command: String,
  apps: Vec<&'static apps::App>,
  pages: Vec<&'static pages::Page>,
  levels: Vec<usize>,
  secs: u64,
  warmup: u64,
  reps: Option<usize>,
  cold: bool,
  machine: String,
  any_power: bool,
  resume: bool,
  rest: Vec<String>,
}

fn parse() -> Result<Args, String> {
  let mut it = std::env::args().skip(1);
  let command = it.next().ok_or_else(|| USAGE.to_owned())?;
  let mut a = Args {
    command,
    apps: apps::APPS.iter().collect(),
    pages: pages::PAGES.iter().collect(),
    levels: vec![1, 8, 32, 64, 128, 256],
    secs: 10,
    warmup: 3,
    reps: None,
    cold: true,
    machine: machine::label(),
    any_power: false,
    resume: false,
    rest: Vec::new(),
  };
  while let Some(flag) = it.next() {
    let mut val = || it.next().ok_or_else(|| format!("{flag} needs a value"));
    match flag.as_str() {
      "--app" => a.apps = val()?.split(',').map(|id| apps::by_id(id).ok_or_else(|| format!("no app {id}"))).collect::<Result<_, _>>()?,
      "--page" => a.pages = val()?.split(',').map(|id| pages::by_id(id).ok_or_else(|| format!("no page {id}"))).collect::<Result<_, _>>()?,
      "--levels" => a.levels = val()?.split(',').map(|c| c.parse().map_err(|_| format!("bad level {c}"))).collect::<Result<_, _>>()?,
      "--secs" => a.secs = val()?.parse().map_err(|_| "bad --secs")?,
      "--warmup" => a.warmup = val()?.parse().map_err(|_| "bad --warmup")?,
      "--reps" => a.reps = Some(val()?.parse().map_err(|_| "bad --reps")?),
      "--no-cold" => a.cold = false,
      "--machine" => a.machine = val()?,
      "--any-power" => a.any_power = true,
      "--resume" => a.resume = true,
      "--help" | "-h" => return Err(USAGE.to_owned()),
      other if !other.starts_with("--") => a.rest.push(other.to_owned()),
      other => return Err(format!("unknown flag {other}\n{USAGE}")),
    }
  }
  Ok(a)
}

fn raw_dir() -> PathBuf {
  apps::root().join("docs/raw")
}

fn runs_dir() -> PathBuf {
  let dir = apps::root().join("runs");
  std::fs::create_dir_all(&dir).expect("runs/");
  dir
}

fn versions(apps: &[&apps::App]) -> Value {
  Value::Object(apps.iter().map(|a| (a.id.to_owned(), json!(a.version()))).collect())
}

/// Refuses to measure outside high power mode unless told to.
fn power_gate(a: &Args) -> Result<(), String> {
  let mode = machine::power_mode();
  if a.any_power || mode.as_deref() == Some("high power") {
    return Ok(());
  }
  Err(format!("power mode is {}; measure in high power mode or pass --any-power", mode.as_deref().unwrap_or("unknown")))
}

fn write_raw(name: &str, value: &Value) -> Result<PathBuf, String> {
  let dir = raw_dir();
  std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
  let path = dir.join(format!("{name}.json"));
  let mut text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
  text.push('\n');
  std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
  Ok(path)
}

fn say_state(when: &str, state: &Value) {
  println!("{when}: {} power mode {}, load {}", state["at"].as_str().unwrap_or("?"), state["power_mode"].as_str().unwrap_or("?"), state["load"]);
}

async fn check(a: &Args) -> Result<(), String> {
  let mut wrong = check::shared_drift();
  let mut seen: Vec<(&str, std::collections::BTreeMap<&'static str, String>)> = Vec::new();
  for app in &a.apps {
    let log = runs_dir().join(format!("{}-check.log", app.id));
    let server = server::Server::start(app, &log).await?;
    let (found, digests) = check::app(app).await;
    server.stop();
    println!("{:<10} {}", app.id, if found.is_empty() { "every page answers as it must".to_owned() } else { format!("{} problems", found.len()) });
    for (page, digest) in &digests {
      println!("           {page:<8} {}", digest.chars().take(90).collect::<String>());
    }
    wrong.extend(found);
    if let Some((first, earlier)) = seen.first() {
      wrong.extend(check::digests_differ((first, earlier), (app.id, &digests)));
    }
    seen.push((app.id, digests));
  }
  if wrong.is_empty() { Ok(()) } else { Err(wrong.join("\n")) }
}

async fn build(a: &Args) -> Result<(), String> {
  power_gate(a)?;
  let start = machine::state();
  say_state("start", &start);
  let reps = a.reps.unwrap_or(3);
  let mut records = Vec::new();
  for app in &a.apps {
    let log = runs_dir().join(format!("{}-build.log", app.id));
    let mut kinds = Vec::new();
    if a.cold {
      kinds.push(true);
    }
    kinds.extend(std::iter::repeat_n(false, reps));
    for (rep, cold) in kinds.into_iter().enumerate() {
      let mut record = builds::time(app, cold, &log)?;
      record["app"] = json!(app.id);
      record["rep"] = json!(rep);
      println!("{:<10} {:<5} {:>8.2} s", app.id, record["kind"].as_str().unwrap_or("?"), record["secs"].as_f64().unwrap_or(0.0));
      records.push(record);
    }
    app.finish().map_err(|e| format!("{}: {e}", app.id))?;
  }
  let end = machine::state();
  say_state("end", &end);
  let path = write_raw("build", &json!({ "machine": a.machine, "start": start, "end": end, "versions": versions(&a.apps), "records": records }))?;
  println!("wrote {}", path.display());
  Ok(())
}

async fn size(a: &Args) -> Result<(), String> {
  let mut records = Vec::new();
  for app in &a.apps {
    let deploy = sizes::deploy(app)?;
    let log = runs_dir().join(format!("{}-size.log", app.id));
    let server = server::Server::start(app, &log).await?;
    let pages = sizes::pages(app).await;
    server.stop();
    let pages = pages?;
    println!("{:<10} deploy tree {} bytes in {} files", app.id, deploy["tree_bytes"], deploy["tree_files"]);
    records.push(json!({ "app": app.id, "deploy": deploy, "pages": pages }));
  }
  let path = write_raw("size", &json!({ "machine": a.machine, "at": machine::now_utc(), "versions": versions(&a.apps), "records": records }))?;
  println!("wrote {}", path.display());
  Ok(())
}

/// Each cell, server and sitting of a run as it lands, one JSON object per line, so a stopped run can be resumed.
fn journal_path() -> PathBuf {
  runs_dir().join("perf.jsonl")
}

/// What an earlier sitting of the run left in the journal.
#[derive(Default)]
struct Journal {
  records: Vec<Value>,
  servers: Vec<Value>,
  sittings: Vec<Value>,
}

fn read_journal(path: &std::path::Path) -> Result<Journal, String> {
  let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
  let mut journal = Journal::default();
  for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
    let entry: Value = serde_json::from_str(line).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1))?;
    if let Some(record) = entry.get("record") {
      journal.records.push(record.clone());
    } else if let Some(server) = entry.get("server") {
      journal.servers.push(server.clone());
    } else if let Some(start) = entry.get("start") {
      journal.sittings.push(json!({ "start": start, "end": Value::Null }));
    } else if let Some(end) = entry.get("end") {
      if let Some(last) = journal.sittings.last_mut() {
        last["end"] = end.clone();
      }
    }
  }
  Ok(journal)
}

fn append(journal: &mut std::fs::File, entry: Value) {
  use std::io::Write;
  let _ = writeln!(journal, "{entry}");
}

/// `runs/perf.jsonl` from a stopped run's printed output: its start line and one record per measured cell, with the fields the output carries.
fn import_log(a: &Args) -> Result<(), String> {
  let source = a.rest.first().ok_or("import-log needs the log to read")?;
  let path = journal_path();
  if path.exists() {
    return Err(format!("{} exists already; it holds a run of its own", path.display()));
  }
  let text = std::fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))?;
  let mut out = std::fs::File::create(&path).map_err(|e| format!("{}: {e}", path.display()))?;
  let mut cells = 0;
  for line in text.lines() {
    if let Some(start) = parse_state(line, "start: ") {
      append(&mut out, json!({ "start": start }));
    } else if let Some(record) = parse_cell(line) {
      append(&mut out, json!({ "record": record }));
      cells += 1;
    }
  }
  println!("wrote {} with {cells} cells", path.display());
  Ok(())
}

/// A state line as `say_state` prints it: `start: <at> power mode <mode>, load <load>`.
fn parse_state(line: &str, prefix: &str) -> Option<Value> {
  let rest = line.strip_prefix(prefix)?;
  let (at, rest) = rest.split_once(" power mode ")?;
  let (mode, load) = rest.split_once(", load ")?;
  Some(json!({ "at": at, "power_mode": mode, "load": serde_json::from_str::<Value>(load).unwrap_or(json!(load)) }))
}

/// A cell line as `run` prints it.
fn parse_cell(line: &str) -> Option<Value> {
  let words: Vec<&str> = line.split_whitespace().collect();
  if words.len() != 18 || words[6] != "req/s" || words[3] != "rep" {
    return None;
  }
  let number = |w: &str| w.trim_end_matches('%').parse::<f64>().ok();
  Some(json!({
    "app": words[0],
    "page": words[1],
    "c": words[2].strip_prefix("c=")?.parse::<u64>().ok()?,
    "rep": words[4].parse::<u64>().ok()?,
    "rps": number(words[5])?,
    "p50_ms": number(words[8])?,
    "p99_ms": number(words[11])?,
    "cpu_pct": number(words[14])?,
    "rss_mb": number(words[16])?,
    "from_log": true,
  }))
}

async fn run(a: &Args) -> Result<(), String> {
  power_gate(a)?;
  let plan = perf::Plan { apps: a.apps.clone(), pages: a.pages.clone(), levels: a.levels.clone(), secs: a.secs, warmup_secs: a.warmup, reps: a.reps.unwrap_or(3) };
  let path = journal_path();
  let earlier = match (a.resume, path.exists()) {
    (true, true) => read_journal(&path)?,
    (true, false) => return Err(format!("--resume: {} does not exist", path.display())),
    (false, true) => return Err(format!("{} holds an earlier run; pass --resume to continue it", path.display())),
    (false, false) => Journal::default(),
  };
  let done: std::collections::BTreeSet<perf::Cell> = earlier.records.iter().filter_map(perf::cell_of).collect();
  let left = plan.cells() - done.iter().filter(|(app, page, c, rep)| a.apps.iter().any(|x| x.id == app) && a.pages.iter().any(|p| p.variants(true).iter().any(|(id, _)| id == page)) && a.levels.contains(c) && *rep < plan.reps).count();
  println!("{} cells, {} measured already, {left} to measure, about {} s of load", plan.cells(), plan.cells() - left, plan.estimate_secs() * left as u64 / plan.cells().max(1) as u64);
  let mut journal = std::fs::File::options().create(true).append(true).open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
  let start = machine::state();
  say_state("start", &start);
  append(&mut journal, json!({ "start": start }));
  let log = runs_dir().join("run-servers.log");
  let server_journal = std::cell::RefCell::new(journal.try_clone().map_err(|e| e.to_string())?);
  let (records, servers) = perf::run(&plan, &log, &done, |r| {
    append(&mut journal, json!({ "record": r }));
    println!(
      "{:<10} {:<8} c={:<4} rep {} {:>10.0} req/s  p50 {:>7.2} ms  p99 {:>7.2} ms  cpu {:>5.0}%  rss {:>6.1} MB",
      r["app"].as_str().unwrap_or("?"),
      r["page"].as_str().unwrap_or("?"),
      r["c"],
      r["rep"],
      r["rps"].as_f64().unwrap_or(0.0),
      r["p50_ms"].as_f64().unwrap_or(0.0),
      r["p99_ms"].as_f64().unwrap_or(0.0),
      r["cpu_pct"].as_f64().unwrap_or(0.0),
      r["rss_mb"].as_f64().unwrap_or(0.0),
    )
  }, |s| append(&mut server_journal.borrow_mut(), json!({ "server": s })))
  .await?;
  let end = machine::state();
  say_state("end", &end);
  append(&mut server_journal.borrow_mut(), json!({ "end": end }));
  let mut sittings = earlier.sittings;
  sittings.push(json!({ "start": start, "end": end }));
  let start = sittings[0]["start"].clone();
  let records: Vec<Value> = earlier.records.into_iter().chain(records).collect();
  let servers: Vec<Value> = earlier.servers.into_iter().chain(servers).collect();
  let path = write_raw(
    "perf",
    &json!({
      "machine": a.machine,
      "start": start,
      "end": end,
      "sittings": sittings,
      "versions": versions(&a.apps),
      "secs": a.secs,
      "warmup_secs": a.warmup,
      "levels": a.levels,
      "reps": plan.reps,
      "client_threads": CLIENT_THREADS,
      "records": records,
      "servers": servers,
    }),
  )?;
  println!("wrote {}", path.display());
  Ok(())
}

fn list(a: &Args) {
  for app in &a.apps {
    println!("app   {:<10} {} on {}", app.id, app.name, app.origin());
  }
  for p in &a.pages {
    println!("page  {:<8} {:<26} {:<6} {}", p.id, p.path, p.render.name(), p.blurb);
    if let Some(native) = p.native {
      println!("        native   {native}, for {}", a.apps.iter().filter(|x| x.native()).map(|x| x.id).collect::<Vec<_>>().join(", "));
    }
  }
  let plan = perf::Plan { apps: a.apps.clone(), pages: a.pages.clone(), levels: a.levels.clone(), secs: a.secs, warmup_secs: a.warmup, reps: a.reps.unwrap_or(3) };
  println!("cells {} levels × {} pages × {} apps with the native forms, {} repetitions: {}, {} s each", a.levels.len(), a.pages.len(), a.apps.len(), plan.reps, plan.cells(), a.secs);
}

/// Worker threads of the load client, which shares the machine with the server it loads.
const CLIENT_THREADS: usize = 4;

fn main() {
  let a = match parse() {
    Ok(a) => a,
    Err(e) => {
      eprintln!("{e}");
      std::process::exit(2);
    }
  };
  let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(CLIENT_THREADS).enable_all().build().expect("a runtime");
  let result = runtime.block_on(async {
    match a.command.as_str() {
      "check" => check(&a).await,
      "build" => build(&a).await,
      "import-log" => import_log(&a),
      "size" => size(&a).await,
      "run" => run(&a).await,
      "report" => report::write(&raw_dir(), &apps::root().join("docs")).map(|written| {
        for path in written {
          println!("wrote {}", path.display());
        }
      }),
      "list" => {
        list(&a);
        Ok(())
      }
      other => Err(format!("unknown command {other}\n{USAGE}")),
    }
  });
  if let Err(e) = result {
    eprintln!("{e}");
    std::process::exit(1);
  }
}

#[cfg(test)]
mod tests {
  use super::{parse_cell, parse_state};

  #[test]
  fn a_printed_cell_and_start_line_read_back() {
    let cell = parse_cell("nuxt       search   c=256 rep 0       3169 req/s  p50   56.50 ms  p99  156.54 ms  cpu   127%  rss  776.6 MB").unwrap();
    assert_eq!((cell["app"].as_str(), cell["page"].as_str(), cell["c"].as_u64(), cell["rep"].as_u64()), (Some("nuxt"), Some("search"), Some(256), Some(0)));
    assert_eq!((cell["rps"].as_f64(), cell["p50_ms"].as_f64(), cell["p99_ms"].as_f64(), cell["cpu_pct"].as_f64(), cell["rss_mb"].as_f64()), (Some(3169.0), Some(56.5), Some(156.54), Some(127.0), Some(776.6)));
    let start = parse_state("start: 2026-11-03T16:53:25Z power mode high power, load 6.23", "start: ").unwrap();
    assert_eq!((start["at"].as_str(), start["power_mode"].as_str(), start["load"].as_f64()), (Some("2026-11-03T16:53:25Z"), Some("high power"), Some(6.23)));
    assert!(parse_cell("1188 cells, about 12474 s of load").is_none());
  }
}
