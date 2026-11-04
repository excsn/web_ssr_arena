//! How long each application takes to build for production.

use std::path::Path;
use std::process::Stdio;
use std::time::{Instant, SystemTime};

use serde_json::{Value, json};

use crate::apps::App;

/// One build of `app`, each step timed, its output appended to `log`. A cold build starts from empty output directories and removes them afterwards; a warm one touches a page first and reuses everything else.
pub fn time(app: &App, cold: bool, log: &Path) -> Result<Value, String> {
  if cold {
    remove_cold(app)?;
  } else {
    let page = app.page_source();
    std::fs::File::options().append(true).open(&page).and_then(|f| f.set_modified(SystemTime::now())).map_err(|e| format!("{}: {e}", page.display()))?;
  }
  let mut steps = Vec::new();
  let mut total = 0.0;
  for mut step in app.build(cold) {
    let out = std::fs::File::options().create(true).append(true).open(log).map_err(|e| format!("{}: {e}", log.display()))?;
    step.command.stdin(Stdio::null()).stdout(out.try_clone().map_err(|e| e.to_string())?).stderr(out);
    let started = Instant::now();
    let status = step.command.status().map_err(|e| format!("{}: {e}", step.label))?;
    let secs = started.elapsed().as_secs_f64();
    if !status.success() {
      return Err(format!("{} {}: {status}; see {}", app.id, step.label, log.display()));
    }
    total += secs;
    steps.push(json!({ "step": step.label, "secs": secs }));
  }
  if cold {
    remove_cold(app)?;
  }
  Ok(json!({ "kind": if cold { "cold" } else { "warm" }, "secs": total, "steps": steps }))
}

fn remove_cold(app: &App) -> Result<(), String> {
  for dir in app.cold_dirs().into_iter().filter(|d| d.exists()) {
    std::fs::remove_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
  }
  Ok(())
}
