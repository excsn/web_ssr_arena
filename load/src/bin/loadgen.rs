//! `loadgen <url> <concurrency> <seconds> [client]`: one run against a server already up, printed as one JSON line. The client is hyper, fibre, fibre-sync or fibre-h2c, hyper by default.

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 4 && args.len() != 5 {
    eprintln!("usage: loadgen <url> <concurrency> <seconds> [hyper|fibre|fibre-sync|fibre-h2c]");
    std::process::exit(2);
  }
  let client = match args.get(4) {
    Some(name) => load::Client::parse(name).unwrap_or_else(|why| {
      eprintln!("{why}");
      std::process::exit(2);
    }),
    None => load::Client::Hyper,
  };
  let concurrency: usize = args[2].parse().expect("a concurrency");
  let seconds: u64 = args[3].parse().expect("seconds");
  match load::run_with(client, &args[1], concurrency, seconds).await {
    Ok(run) => {
      for why in &run.errors {
        eprintln!("{why}");
      }
      println!("{}", run.json());
    }
    Err(why) => {
      eprintln!("{why}");
      std::process::exit(1);
    }
  }
}
