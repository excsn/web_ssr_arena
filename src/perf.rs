//! Requests per second, latency, memory and CPU under load, one fixed-duration cell at a time.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::{Value, json};

use crate::apps::App;
use crate::check;
use crate::pages::Page;
use crate::server::Server;

pub struct Plan<'a> {
  pub apps: Vec<&'a App>,
  pub pages: Vec<&'a Page>,
  pub levels: Vec<usize>,
  pub secs: u64,
  pub warmup_secs: u64,
  pub reps: usize,
}

impl Plan<'_> {
  /// Pages times apps, each native form counted as a page of its own.
  fn measured_pages(&self) -> usize {
    self.apps.iter().map(|a| self.pages.iter().map(|p| p.variants(a.native()).len()).sum::<usize>()).sum()
  }

  pub fn cells(&self) -> usize {
    self.measured_pages() * self.levels.len() * self.reps
  }

  /// Wall time of the measured cells and warm-ups, without server starts.
  pub fn estimate_secs(&self) -> u64 {
    self.measured_pages() as u64 * (self.warmup_secs + self.levels.len() as u64 * self.secs) * self.reps as u64
  }
}

/// A measured cell: app, page, concurrency and repetition.
pub type Cell = (String, String, usize, usize);

/// The cell a record measured.
pub fn cell_of(record: &Value) -> Option<Cell> {
  Some((record["app"].as_str()?.to_owned(), record["page"].as_str()?.to_owned(), record["c"].as_u64()? as usize, record["rep"].as_u64()? as usize))
}

/// Runs the cells of `plan` not in `done`, the applications alternating within each repetition and their order flipping between repetitions. Each repetition starts every server afresh; a server with no cell left in its repetition is not started.
pub async fn run(plan: &Plan<'_>, log: &Path, done: &BTreeSet<Cell>, mut on_record: impl FnMut(&Value), mut on_server: impl FnMut(&Value)) -> Result<(Vec<Value>, Vec<Value>), String> {
  let mut records = Vec::new();
  let mut servers = Vec::new();
  let mut first: Option<(&str, std::collections::BTreeMap<&'static str, String>)> = None;
  let missing = |app: &str, page: &str, c: usize, rep: usize| !done.contains(&(app.to_owned(), page.to_owned(), c, rep));
  for rep in 0..plan.reps {
    let mut order = plan.apps.clone();
    if rep % 2 == 1 {
      order.reverse();
    }
    for app in order {
      if !plan.pages.iter().flat_map(|page| page.variants(app.native())).any(|(id, _)| plan.levels.iter().any(|&c| missing(app.id, &id, c, rep))) {
        continue;
      }
      let server = Server::start(app, log).await?;
      let cold_ms = server.cold_ms;
      let idle_mb = server.rss_mb();
      let (mut wrong, digests) = check::app(app).await;
      match &first {
        Some((id, earlier)) => wrong.extend(check::digests_differ((id, earlier), (app.id, &digests))),
        None => first = Some((app.id, digests)),
      }
      if !wrong.is_empty() {
        server.stop();
        return Err(format!("{} answers wrongly:\n  {}", app.id, wrong.join("\n  ")));
      }
      for (page, (id, path)) in plan.pages.iter().flat_map(|page| page.variants(app.native()).into_iter().map(move |v| (page, v))) {
        let levels: Vec<usize> = plan.levels.iter().copied().filter(|&c| missing(app.id, &id, c, rep)).collect();
        if levels.is_empty() {
          continue;
        }
        let url = format!("{}{path}", app.origin());
        if plan.warmup_secs > 0 {
          load::run(&url, 16, plan.warmup_secs).await?;
        }
        for c in levels {
          let cpu_before = server.cpu_s();
          let run = load::run(&url, c, plan.secs).await?;
          let cpu_after = server.cpu_s();
          let cpu_pct = cpu_before.zip(cpu_after).map(|(a, b)| 100.0 * (b - a) / run.wall_s);
          let expected_non2xx = if page.status == 200 { 0 } else { run.done as u64 };
          let record = json!({
            "app": app.id,
            "page": id,
            "c": c,
            "rep": rep,
            "rps": run.rps,
            "done": run.done,
            "p50_ms": run.p50_ms,
            "p95_ms": run.p95_ms,
            "p99_ms": run.p99_ms,
            "max_ms": run.max_ms,
            "unexpected_status": run.non2xx.abs_diff(expected_non2xx),
            "failed": run.failed,
            "connections": run.connections,
            "cpu_pct": cpu_pct,
            "rss_mb": server.rss_mb(),
          });
          on_record(&record);
          records.push(record);
        }
      }
      let usage = server.stop();
      let record = json!({ "app": app.id, "rep": rep, "cold_ms": cold_ms, "idle_rss_mb": idle_mb, "peak_rss_mb": usage.peak_rss_mb, "cpu_s": usage.cpu_s });
      on_server(&record);
      servers.push(record);
    }
  }
  Ok((records, servers))
}

#[cfg(test)]
mod tests {
  use super::Plan;
  use crate::apps::APPS;
  use crate::pages::PAGES;

  #[test]
  fn the_native_forms_are_cells_of_their_own_for_fsr_only() {
    let plan = Plan { apps: APPS.iter().collect(), pages: PAGES.iter().collect(), levels: vec![1, 8, 32, 64, 128, 256], secs: 10, warmup_secs: 3, reps: 3 };
    assert_eq!(plan.cells(), 7 * 11 * 6 * 3 + 3 * 6 * 3);
    let primes = crate::pages::by_id("primes").unwrap();
    assert_eq!(primes.variants(true), vec![("primes".to_owned(), "/compute/primes?n=50000"), ("primes.native".to_owned(), "/compute/native/primes?n=50000")]);
    assert_eq!(primes.variants(false).len(), 1);
    assert_eq!(crate::pages::by_id("table").unwrap().variants(true).len(), 1);
  }
}
