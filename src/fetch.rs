//! One GET on a connection of its own, read whole.

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::client::conn::http1::handshake;
use hyper::{Request, Uri, header};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub struct Answer {
  pub status: u16,
  pub content_type: String,
  pub body: Vec<u8>,
}

impl Answer {
  /// The body as text, with the `<!-- -->` separators React writes between adjacent text nodes taken out.
  pub fn text(&self) -> String {
    String::from_utf8_lossy(&self.body).replace("<!-- -->", "")
  }
}

pub async fn get(url: &str) -> Result<Answer, String> {
  let uri: Uri = url.parse().map_err(|e| format!("{url}: {e}"))?;
  let authority = uri.authority().ok_or_else(|| format!("{url}: no host and port"))?.to_string();
  let path = uri.path_and_query().map_or("/", |p| p.as_str()).to_owned();
  let stream = TcpStream::connect(&authority).await.map_err(|e| format!("{url}: connect: {e}"))?;
  let (mut sender, connection) = handshake(TokioIo::new(stream)).await.map_err(|e| format!("{url}: handshake: {e}"))?;
  tokio::spawn(connection);
  let request = Request::get(path).header(header::HOST, authority).body(Empty::<Bytes>::new()).map_err(|e| e.to_string())?;
  let response = sender.send_request(request).await.map_err(|e| format!("{url}: send: {e}"))?;
  let status = response.status().as_u16();
  let content_type = response.headers().get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).unwrap_or("").to_owned();
  let body = response.into_body().collect().await.map_err(|e| format!("{url}: body: {e}"))?.to_bytes().to_vec();
  Ok(Answer { status, content_type, body })
}
