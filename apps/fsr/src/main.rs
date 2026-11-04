use std::sync::Arc;

use snapfire_fsr_host::Host;

#[allow(dead_code)]
mod catalog;
#[allow(dead_code)]
mod compute;
mod native;

mod bodies {
  include!(concat!(env!("CARGO_MANIFEST_DIR"), "/app/generated/bodies.rs"));
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let host = Host::from_cwd().map(|b| b.bodies(bodies::BODIES).native("compute", Arc::new(native::Compute))).and_then(|b| b.build()).map_err(std::io::Error::other)?;
  if std::env::args().nth(1).as_deref() == Some("prerender") {
    let out = host.report().prerender.clone().ok_or_else(|| std::io::Error::other("config/app.toml sets no server.prerender"))?;
    for (pattern, file) in host.prerender(&out).await.map_err(std::io::Error::other)? {
      println!("{pattern} -> {}", file.display());
    }
    return Ok(());
  }
  print!("{}", host.report());
  let listen = host.listen().to_owned();
  Arc::new(host).serve(&listen).await
}
