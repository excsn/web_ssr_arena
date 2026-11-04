//! What each application costs on disk and over the wire.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Write;

use flate2::Compression;
use flate2::write::GzEncoder;
use serde_json::{Value, json};

use crate::apps::{self, App};
use crate::fetch;
use crate::pages::{PAGES, Render};

pub fn gzip_len(bytes: &[u8]) -> u64 {
  let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
  encoder.write_all(bytes).expect("gzip into memory");
  encoder.finish().expect("gzip into memory").len() as u64
}

/// The deploy tree's bytes and files, the server binary and the runtime the tree needs beside it.
pub fn deploy(app: &App) -> Result<Value, String> {
  let out = apps::root().join("runs/deploy").join(app.id);
  if out.exists() {
    std::fs::remove_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
  }
  app.deploy(&out)?;
  let (bytes, files) = apps::tree_size(&out);
  let binary = app.binary().and_then(|name| std::fs::metadata(out.join(name)).ok()).map(|m| m.len());
  let stripped = app.binary().and_then(|name| stripped_len(&out.join(name)));
  let runtime = app.runtime();
  Ok(json!({
    "tree_bytes": bytes,
    "tree_files": files,
    "binary_bytes": binary,
    "stripped_binary_bytes": stripped,
    "runtime": runtime.as_ref().map(|(name, _)| name.clone()),
    "runtime_bytes": runtime.map(|(_, bytes)| bytes),
  }))
}

/// The size of `binary` with its symbols and debug information stripped, written beside it and removed.
fn stripped_len(binary: &std::path::Path) -> Option<u64> {
  let copy = binary.with_extension("stripped");
  let ok = std::process::Command::new("strip").arg("-o").arg(&copy).arg(binary).status().ok()?.success();
  let len = std::fs::metadata(&copy).ok().map(|m| m.len());
  let _ = std::fs::remove_file(&copy);
  if ok { len } else { None }
}

#[derive(Default)]
struct Kinds {
  html: (u64, u64),
  css: (u64, u64),
  js: (u64, u64),
  wasm: (u64, u64),
  later_js: (u64, u64),
  requests: u64,
  later_requests: u64,
}

fn add(to: &mut (u64, u64), body: &[u8]) {
  to.0 += body.len() as u64;
  to.1 += gzip_len(body);
}

/// The bytes a browser fetches to show `page`: the document, its stylesheets, the WebAssembly it preloads and the scripts it loads, following the static imports of module scripts. Modules reached only by a dynamic `import()` are counted apart, as loaded later.
pub async fn page(app: &App, path: &str) -> Result<Value, String> {
  let origin = app.origin();
  let document = fetch::get(&format!("{origin}{path}")).await?;
  let html = String::from_utf8_lossy(&document.body).into_owned();
  let mut kinds = Kinds::default();
  add(&mut kinds.html, &document.body);
  kinds.requests += 1;

  let tags = Tags::read(&html);
  let mut seen = BTreeSet::new();
  for href in &tags.styles {
    let url = resolve(&origin, &format!("{origin}{path}"), href, &tags.import_map).ok_or_else(|| format!("{href}: cannot resolve"))?;
    if seen.insert(url.clone()) {
      let body = fetch_ok(&url).await?;
      add(&mut kinds.css, &body);
      kinds.requests += 1;
    }
  }
  for href in &tags.wasm {
    let url = resolve(&origin, &format!("{origin}{path}"), href, &tags.import_map).ok_or_else(|| format!("{href}: cannot resolve"))?;
    if seen.insert(url.clone()) {
      let body = fetch_ok(&url).await?;
      add(&mut kinds.wasm, &body);
      kinds.requests += 1;
    }
  }
  let mut queue: VecDeque<(String, bool, bool)> = VecDeque::new();
  for (src, module) in &tags.scripts {
    let url = resolve(&origin, &format!("{origin}{path}"), src, &tags.import_map).ok_or_else(|| format!("{src}: cannot resolve"))?;
    match queue.iter_mut().find(|(queued, _, _)| *queued == url) {
      Some(queued) => queued.1 |= *module,
      None => queue.push_back((url, *module, false)),
    }
  }
  let mut later = BTreeSet::new();
  while let Some((url, module, deferred)) = queue.pop_front() {
    if seen.contains(&url) || (deferred && later.contains(&url)) {
      continue;
    }
    let body = fetch_ok(&url).await?;
    if deferred {
      later.insert(url.clone());
      add(&mut kinds.later_js, &body);
      kinds.later_requests += 1;
    } else {
      seen.insert(url.clone());
      add(&mut kinds.js, &body);
      kinds.requests += 1;
    }
    if !module {
      continue;
    }
    let text = String::from_utf8_lossy(&body);
    for (specifier, dynamic) in imports(&text) {
      if let Some(next) = resolve(&origin, &url, &specifier, &tags.import_map) {
        queue.push_back((next, true, deferred || dynamic));
      }
    }
    if deferred {
      continue;
    }
    for literal in wasm_literals(&text) {
      let Some(wasm) = resolve(&origin, &url, &literal, &BTreeMap::new()) else { continue };
      if seen.contains(&wasm) {
        continue;
      }
      let answer = fetch::get(&wasm).await?;
      if answer.status == 200 {
        seen.insert(wasm);
        add(&mut kinds.wasm, &answer.body);
        kinds.requests += 1;
      }
    }
  }
  later.retain(|url| !seen.contains(url));
  Ok(json!({
    "requests": kinds.requests,
    "html": kinds.html.0, "html_gz": kinds.html.1,
    "css": kinds.css.0, "css_gz": kinds.css.1,
    "js": kinds.js.0, "js_gz": kinds.js.1,
    "wasm": kinds.wasm.0, "wasm_gz": kinds.wasm.1,
    "later_requests": kinds.later_requests,
    "later_js": kinds.later_js.0, "later_js_gz": kinds.later_js.1,
  }))
}

/// Every HTML page of `app`, against a server already up.
pub async fn pages(app: &App) -> Result<BTreeMap<String, Value>, String> {
  let mut out = BTreeMap::new();
  for p in PAGES.iter().filter(|p| p.render != Render::Api && p.status == 200) {
    out.insert(p.id.to_owned(), page(app, p.path).await?);
  }
  Ok(out)
}

async fn fetch_ok(url: &str) -> Result<Vec<u8>, String> {
  let answer = fetch::get(url).await?;
  if answer.status != 200 {
    return Err(format!("{url}: status {}", answer.status));
  }
  Ok(answer.body)
}

#[derive(Default, Debug)]
pub struct Tags {
  pub styles: Vec<String>,
  /// Source and whether it is a module.
  pub scripts: Vec<(String, bool)>,
  /// WebAssembly modules the document preloads.
  pub wasm: Vec<String>,
  pub import_map: BTreeMap<String, String>,
}

impl Tags {
  pub fn read(html: &str) -> Tags {
    let mut tags = Tags::default();
    let mut rest = html;
    while let Some(start) = rest.find('<') {
      rest = &rest[start..];
      let end = rest.find('>').map_or(rest.len(), |e| e + 1);
      let tag = &rest[..end];
      let lower = tag.to_ascii_lowercase();
      if lower.starts_with("<link") {
        let rel = attr(tag, "rel").unwrap_or_default().to_ascii_lowercase();
        let href = attr(tag, "href");
        match (rel.as_str(), href) {
          ("stylesheet", Some(href)) => tags.styles.push(href),
          ("modulepreload", Some(href)) => tags.scripts.push((href, true)),
          ("preload", Some(href)) if attr(tag, "as").as_deref() == Some("script") => tags.scripts.push((href, false)),
          ("preload", Some(href)) if attr(tag, "type").as_deref() == Some("application/wasm") => tags.wasm.push(href),
          _ => {}
        }
      } else if lower.starts_with("<script") {
        let kind = attr(tag, "type").unwrap_or_default();
        if kind == "importmap" {
          let body_end = rest.find("</script>").unwrap_or(rest.len());
          if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&rest[end..body_end]) {
            if let Some(Value::Object(imports)) = map.get("imports") {
              tags.import_map.extend(imports.iter().filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_owned()))));
            }
          }
        } else if let Some(src) = attr(tag, "src") {
          if !lower.contains("nomodule") {
            tags.scripts.push((src, kind == "module"));
          }
        }
      }
      rest = &rest[end..];
    }
    tags
  }
}

fn attr(tag: &str, name: &str) -> Option<String> {
  let lower = tag.to_ascii_lowercase();
  let mut from = 0;
  while let Some(at) = lower[from..].find(name) {
    let at = from + at;
    from = at + name.len();
    let before = lower[..at].chars().last();
    if !matches!(before, Some(' ' | '\n' | '\t')) {
      continue;
    }
    let after = lower[from..].trim_start();
    let Some(value) = after.strip_prefix('=') else { continue };
    let offset = tag.len() - value.len();
    let value = &tag[offset..].trim_start();
    let quote = value.chars().next()?;
    if quote == '"' || quote == '\'' {
      return value[1..].find(quote).map(|e| value[1..1 + e].to_owned());
    }
    return Some(value.split(|c: char| c.is_whitespace() || c == '>').next()?.to_owned());
  }
  None
}

/// The specifiers a module imports, with whether each is a dynamic `import()`.
pub fn imports(source: &str) -> Vec<(String, bool)> {
  let mut out = Vec::new();
  let bytes = source.as_bytes();
  let quoted = |at: usize| -> Option<String> {
    let rest = source[at..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
      return None;
    }
    rest[1..].find(quote).map(|e| rest[1..1 + e].to_owned())
  };
  for (at, _) in source.match_indices("from") {
    let before = at.checked_sub(1).map(|i| bytes[i]);
    if before.is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$') {
      continue;
    }
    if let Some(s) = quoted(at + 4) {
      out.push((s, false));
    }
  }
  for (at, _) in source.match_indices("import") {
    let before = at.checked_sub(1).map(|i| bytes[i]);
    if before.is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'.') {
      continue;
    }
    let rest = source[at + 6..].trim_start();
    if let Some(call) = rest.strip_prefix('(') {
      if let Some(s) = quoted(source.len() - call.len()) {
        out.push((s, true));
      }
    } else if let Some(s) = quoted(at + 6) {
      out.push((s, false));
    }
  }
  out
}

/// The quoted strings in a module that name a `.wasm` file. A loader may hold a fallback name it never fetches; the caller counts only those the server answers.
pub fn wasm_literals(source: &str) -> Vec<String> {
  let mut out = Vec::new();
  for (at, _) in source.match_indices(".wasm") {
    let after = source[at + 5..].chars().next();
    let Some(quote) = after.filter(|c| matches!(c, '"' | '\'' | '`')) else { continue };
    let Some(open) = source[..at].rfind(quote) else { continue };
    let literal = &source[open + 1..at + 5];
    if !literal.is_empty() && !literal.contains(char::is_whitespace) && !out.iter().any(|seen: &String| seen == literal) {
      out.push(literal.to_owned());
    }
  }
  out
}

/// `specifier` as an absolute URL: through the import map when bare, against `base` when relative.
pub fn resolve(origin: &str, base: &str, specifier: &str, map: &BTreeMap<String, String>) -> Option<String> {
  if specifier.starts_with("http://") || specifier.starts_with("https://") {
    return Some(specifier.to_owned());
  }
  if specifier.starts_with('/') {
    return Some(format!("{origin}{}", specifier.replace("/./", "/")));
  }
  if specifier.starts_with("./") || specifier.starts_with("../") {
    let path = base.strip_prefix(origin)?;
    let path = path.split(['?', '#']).next()?;
    let mut parts: Vec<&str> = path.split('/').collect();
    parts.pop();
    for piece in specifier.split('/') {
      match piece {
        "." => {}
        ".." => {
          if parts.len() > 1 {
            parts.pop();
          }
        }
        other => parts.push(other),
      }
    }
    return Some(format!("{origin}{}", parts.join("/")));
  }
  if let Some(target) = map.get(specifier) {
    return resolve(origin, base, target, &BTreeMap::new());
  }
  let (prefix, target) = map.iter().filter(|(k, _)| k.ends_with('/') && specifier.starts_with(k.as_str())).max_by_key(|(k, _)| k.len())?;
  resolve(origin, base, &format!("{target}{}", &specifier[prefix.len()..]), &BTreeMap::new())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_tags_a_document_loads_are_read() {
    let html = r#"<link rel="stylesheet" href="/a.css"><script type="importmap">{"imports":{"react":"/v/react.mjs"}}</script><script type="module" src="/m.js"></script><script src="/c.js" async=""></script><script src="/legacy.js" noModule=""></script><link rel="preload" as="script" fetchPriority="low" href="/p.js"/><link rel="preload" href="/pkg/a.wasm" as="fetch" type="application/wasm">"#;
    let tags = Tags::read(html);
    assert_eq!(tags.styles, vec!["/a.css"]);
    assert_eq!(tags.scripts, vec![("/m.js".to_owned(), true), ("/c.js".to_owned(), false), ("/p.js".to_owned(), false)]);
    assert_eq!(tags.wasm, vec!["/pkg/a.wasm"]);
    assert_eq!(tags.import_map.get("react").map(String::as_str), Some("/v/react.mjs"));
  }

  #[test]
  fn static_and_dynamic_imports_are_told_apart() {
    let source = r#"import{a as b}from"./x.js";import "react";const m=()=>import("./lazy.js");export{c}from'../y.js';const fromage=1;"#;
    assert_eq!(imports(source), vec![("./x.js".to_owned(), false), ("../y.js".to_owned(), false), ("react".to_owned(), false), ("./lazy.js".to_owned(), true)]);
  }

  #[test]
  fn wasm_named_in_a_module_is_found() {
    let source = r#"s===void 0&&(s=new URL("app_bg.wasm",import.meta.url));dt({module_or_path:"/./assets/app_bg-dxh1.wasm"});e.wasm_bindgen_x()"#;
    assert_eq!(wasm_literals(source), vec!["app_bg.wasm".to_owned(), "/./assets/app_bg-dxh1.wasm".to_owned()]);
    assert_eq!(resolve("http://h", "http://h/assets/app.js", "/./assets/app_bg-dxh1.wasm", &BTreeMap::new()).as_deref(), Some("http://h/assets/app_bg-dxh1.wasm"));
  }

  #[test]
  fn specifiers_resolve_against_the_module_and_the_map() {
    let map = BTreeMap::from([("react".to_owned(), "/v/react.mjs".to_owned()), ("lib/".to_owned(), "/v/lib/".to_owned())]);
    let o = "http://h";
    assert_eq!(resolve(o, "http://h/js/app/main.js", "./ui/a.js", &map).as_deref(), Some("http://h/js/app/ui/a.js"));
    assert_eq!(resolve(o, "http://h/js/app/main.js", "../b.js", &map).as_deref(), Some("http://h/js/b.js"));
    assert_eq!(resolve(o, "http://h/js/app/main.js", "react", &map).as_deref(), Some("http://h/v/react.mjs"));
    assert_eq!(resolve(o, "http://h/js/app/main.js", "lib/x.js", &map).as_deref(), Some("http://h/v/lib/x.js"));
  }
}
