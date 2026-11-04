//! A server started for a measurement and stopped for its resource use.

use std::fs::File;
use std::path::Path;
use std::process::{Child, Stdio};
use std::time::{Duration, Instant};

use crate::apps::App;

const FIRST_200_LIMIT: Duration = Duration::from_secs(60);

pub struct Server {
  child: Child,
  pub cold_ms: f64,
}

pub struct Usage {
  pub peak_rss_mb: f64,
  pub cpu_s: f64,
}

impl Server {
  /// Starts `app` with its output appended to `log` and waits for the first 200 from `/`.
  pub async fn start(app: &App, log: &Path) -> Result<Server, String> {
    if load::status(&format!("{}/", app.origin())).await.is_ok() {
      return Err(format!("something already answers on port {}", app.port));
    }
    let out = File::options().create(true).append(true).open(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let started = Instant::now();
    let mut command = app.serve();
    command.stdin(Stdio::null()).stdout(out.try_clone().map_err(|e| e.to_string())?).stderr(out);
    let mut child = command.spawn().map_err(|e| format!("{}: {e}", app.id))?;
    let url = format!("{}/", app.origin());
    while started.elapsed() < FIRST_200_LIMIT {
      if let Ok(Some(status)) = child.try_wait() {
        return Err(format!("{} exited with {status} before answering; see {}", app.id, log.display()));
      }
      if load::status(&url).await == Ok(200) {
        return Ok(Server { child, cold_ms: started.elapsed().as_secs_f64() * 1000.0 });
      }
      tokio::time::sleep(Duration::from_millis(5)).await;
    }
    let _ = Server { child, cold_ms: 0.0 }.stop();
    Err(format!("no 200 from {url} within {} s; see {}", FIRST_200_LIMIT.as_secs(), log.display()))
  }

  /// The server's resident set now, in MB.
  pub fn rss_mb(&self) -> Option<f64> {
    let kb: f64 = crate::machine::output("ps", &["-o", "rss=", "-p", &self.child.id().to_string()])?.parse().ok()?;
    Some(kb / 1024.0)
  }

  /// SIGTERM, then the peak resident set and the user plus system CPU time over the server's life.
  pub fn stop(self) -> Usage {
    let pid = self.child.id() as libc::pid_t;
    let mut status = 0;
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe {
      libc::kill(pid, libc::SIGTERM);
      libc::wait4(pid, &mut status, 0, &mut usage);
    }
    // ru_maxrss is bytes on macOS and kilobytes on Linux.
    let rss_bytes = if cfg!(target_os = "macos") { usage.ru_maxrss as f64 } else { usage.ru_maxrss as f64 * 1024.0 };
    let seconds = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 / 1e6;
    Usage { peak_rss_mb: rss_bytes / 1_048_576.0, cpu_s: seconds(usage.ru_utime) + seconds(usage.ru_stime) }
  }

  /// CPU seconds the server has used so far.
  pub fn cpu_s(&self) -> Option<f64> {
    let text = crate::machine::output("ps", &["-o", "time=", "-p", &self.child.id().to_string()])?;
    parse_cpu_time(&text)
  }
}

/// `ps -o time=` as seconds: `[[dd-]hh:]mm:ss.ss`.
fn parse_cpu_time(text: &str) -> Option<f64> {
  let (days, rest) = match text.split_once('-') {
    Some((d, rest)) => (d.trim().parse::<f64>().ok()?, rest),
    None => (0.0, text),
  };
  let mut seconds = 0.0;
  for part in rest.trim().split(':') {
    seconds = seconds * 60.0 + part.parse::<f64>().ok()?;
  }
  Some(days * 86_400.0 + seconds)
}

#[cfg(test)]
mod tests {
  use super::parse_cpu_time;

  #[test]
  fn cpu_time_reads_every_form_ps_writes() {
    assert_eq!(parse_cpu_time("0:01.50"), Some(1.5));
    assert_eq!(parse_cpu_time("1:02:03.00"), Some(3723.0));
    assert_eq!(parse_cpu_time("1-00:00:01.00"), Some(86_401.0));
  }
}
