//! The machine state a run records beside its numbers.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

pub fn state() -> Value {
  json!({ "at": now_utc(), "power_mode": power_mode(), "load": load_average() })
}

pub fn label() -> String {
  let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0);
  let model = output("sysctl", &["-n", "machdep.cpu.brand_string"]).unwrap_or_else(|| std::env::consts::ARCH.to_owned());
  format!("{model} ({cores} cores)")
}

pub fn power_mode() -> Option<String> {
  let text = output("pmset", &["-g"])?;
  let mode = text.lines().find(|l| l.trim_start().starts_with("powermode"))?.split_whitespace().nth(1)?;
  Some(
    match mode {
      "0" => "automatic",
      "1" => "low power",
      "2" => "high power",
      other => other,
    }
    .to_owned(),
  )
}

pub fn load_average() -> Option<f64> {
  output("sysctl", &["-n", "vm.loadavg"])?.split_whitespace().find_map(|field| field.parse().ok())
}

pub fn output(program: &str, args: &[&str]) -> Option<String> {
  let out = Command::new(program).args(args).output().ok().filter(|o| o.status.success())?;
  Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

pub fn now_utc() -> String {
  let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
  let (days, rem) = (secs / 86_400, secs % 86_400);
  let (year, month, day) = civil_from_days(days as i64);
  format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}

/// Howard Hinnant's days-to-civil conversion for the proleptic Gregorian calendar.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
  let z = days + 719_468;
  let era = z.div_euclid(146_097);
  let doe = z.rem_euclid(146_097);
  let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
  let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
  let year = yoe + era * 400 + i64::from(month <= 2);
  (year, month, day)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn two_different_uncommitted_trees_have_different_labels() {
    let dir = std::env::temp_dir().join(format!("arena-commit-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let git = |args: &[&str]| assert!(Command::new("git").arg("-C").arg(&dir).args(["-c", "user.name=t", "-c", "user.email=t@t"]).args(args).output().unwrap().status.success());
    git(&["init", "-q"]);
    std::fs::write(dir.join("a"), "1").unwrap();
    git(&["add", "a"]);
    git(&["commit", "-q", "-m", "a"]);
    let clean = commit(&dir).unwrap();
    std::fs::write(dir.join("a"), "2").unwrap();
    let edited = commit(&dir).unwrap();
    std::fs::write(dir.join("a"), "3").unwrap();
    let edited_again = commit(&dir).unwrap();
    std::fs::write(dir.join("b"), "new").unwrap();
    let with_new_file = commit(&dir).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(edited.starts_with(&format!("{clean}+dirty.")), "{edited}");
    assert_ne!(edited, edited_again);
    assert_ne!(edited_again, with_new_file);
  }

  #[test]
  fn civil_dates() {
    assert_eq!(civil_from_days(0), (1970, 1, 1));
    assert_eq!(civil_from_days(19_723), (2024, 1, 1));
  }
}
