//! Every application answers every page with what the page has to hold, before anything is measured.

use std::collections::BTreeMap;

use crate::apps::{self, App};
use crate::fetch;
use crate::pages::{PAGES, Page, Render};

/// The copies under each application that differ from `shared/`.
pub fn shared_drift() -> Vec<String> {
  let root = apps::root();
  let mut drift = Vec::new();
  for app in apps::APPS {
    for (name, copy) in app.shared_copies() {
      let source = std::fs::read(root.join("shared").join(&name)).unwrap_or_default();
      if std::fs::read(app.dir().join(&copy)).ok().as_deref() != Some(source.as_slice()) {
        drift.push(format!("apps/{}/{copy} differs from shared/{name}", app.id));
      }
    }
  }
  drift
}

/// The text of the body's `<p class="digest">`, without the comments a framework writes into it to mark nodes for hydration.
pub fn digest(text: &str) -> Option<String> {
  let start = text.find("<p class=\"digest\">")? + "<p class=\"digest\">".len();
  let end = text[start..].find("</p>")?;
  let mut inner = &text[start..start + end];
  let mut out = String::new();
  while let Some(open) = inner.find("<!--") {
    out.push_str(&inner[..open]);
    inner = inner[open..].find("-->").map_or("", |close| &inner[open + close + 3..]);
  }
  out.push_str(inner);
  Some(out)
}

#[cfg(test)]
mod tests {
  use super::digest;

  #[test]
  fn a_digest_is_read_without_hydration_comments() {
    assert_eq!(digest(r#"<p class="digest"><!--node-id13-->primes 10 4 17 2,3,5,7<!--#--></p>"#).as_deref(), Some("primes 10 4 17 2,3,5,7"));
    assert_eq!(digest(r#"<p class="digest">table 500 meadow-bottle</p>"#).as_deref(), Some("table 500 meadow-bottle"));
  }
}

/// What is wrong with `app`'s answer to `page` at `path` against a server already up, with the digest it wrote.
pub async fn page(app: &App, page: &Page, path: &str) -> (Vec<String>, Option<String>) {
  let url = format!("{}{path}", app.origin());
  let answer = match fetch::get(&url).await {
    Ok(answer) => answer,
    Err(e) => return (vec![e], None),
  };
  let mut wrong = Vec::new();
  if answer.status != page.status {
    wrong.push(format!("{url}: status {} where {} was expected", answer.status, page.status));
  }
  let wanted_type = if page.render == Render::Api { "application/json" } else { "text/html" };
  if !answer.content_type.starts_with(wanted_type) {
    wrong.push(format!("{url}: content type {:?} where {wanted_type} was expected", answer.content_type));
  }
  let text = answer.text();
  for held in page.holds {
    if !text.contains(held) {
      wrong.push(format!("{url}: the body does not hold {held:?}"));
    }
  }
  let cards = text.matches("class=\"card\"").count();
  if cards != page.cards {
    wrong.push(format!("{url}: {cards} product cards where {} were expected", page.cards));
  }
  let written = digest(&text);
  if page.digest && written.is_none() {
    wrong.push(format!("{url}: the body writes no digest"));
  }
  (wrong, written)
}

/// Every page of `app`, against a server already up, and the digest of each page that writes one.
pub async fn app(app: &App) -> (Vec<String>, BTreeMap<&'static str, String>) {
  let mut wrong = Vec::new();
  let mut digests = BTreeMap::new();
  for p in PAGES {
    let (found, written) = page(app, p, p.path).await;
    wrong.extend(found);
    if let (true, Some(written)) = (p.digest, written) {
      digests.insert(p.id, written);
    }
    for (id, path) in p.variants(app.native()).into_iter().skip(1) {
      let (found, written) = page(app, p, path).await;
      wrong.extend(found);
      if p.digest && written.as_ref() != digests.get(p.id) {
        wrong.push(format!("{id}: wrote {written:?} where {} wrote {:?}", p.id, digests.get(p.id)));
      }
    }
  }
  (wrong, digests)
}

/// The pages whose digest differs between two apps.
pub fn digests_differ(a: (&str, &BTreeMap<&'static str, String>), b: (&str, &BTreeMap<&'static str, String>)) -> Vec<String> {
  let mut wrong = Vec::new();
  for (page, left) in a.1 {
    match b.1.get(page) {
      Some(right) if right == left => {}
      right => wrong.push(format!("{page}: {} wrote {left:?} and {} wrote {right:?}", a.0, b.0)),
    }
  }
  wrong
}
