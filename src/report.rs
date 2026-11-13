//! `docs/README.md` and `docs/raw/results.json` from the raw files the measuring commands write.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::apps::{self, APPS};
use crate::pages::PAGES;

const SUMMARY: &str = "The same storefront written in SnapFire FSR, Next.js, SvelteKit, Nuxt, Astro, Leptos and Dioxus, built for production and served on one machine: build time, deploy size, bytes per page and requests per second.";

const READING: &str = "Higher is better for requests per second. Lower is better for latency, memory, build time and size. Bold marks the best figure in a row and every figure within 5% of it.";

const METHOD: &[&str] = &[
  "Every application is the same storefront: 48 products in a module each imports, a landing page and a counter page built once at build time and a catalog, product and search page rendered for each request because each reads the query or a parameter. The data, the compute code and the stylesheet are shared byte for byte, with the data and compute code written once in TypeScript and once in Rust for Leptos and Dioxus. The views are shared as TSX by FSR and Next.js and written once each as Svelte, Vue, Leptos and Dioxus components for those four, with the same markup.",
  "Three compute pages run a loader that loops and allocates per request, the work a JIT is built for: a sieve of Eratosthenes, an aggregation of generated orders sorted with a comparator and a word count over a built-up text. A table page renders 500 rows from simple data. Each writes a digest of what it computed and a run refuses to start when the applications' digests differ. FSR is measured on the three compute pages a second time with the loader calling the Rust that Leptos and Dioxus run through `#[native]`. That figure is shown under its own.",
  "FSR is built with `fsr build app` and then `cargo build --release` under `RELEASE_ENV=production`, which compiles each loader the generator can write into the binary, then prerenders its fixed routes. Its render memo (`[cache]`) is off, so every live request renders. Next.js is built with `next build` and served from its `standalone` output with `node server.js`; the live pages set `dynamic = \"force-dynamic\"`. SvelteKit is built with `vite build` and `@sveltejs/adapter-node` and served with `node build`; each live page computes in a `+page.server.ts` load. Nuxt is built with `nuxt build` and served with `node .output/server/index.mjs`; each live page computes in `useAsyncData`. Astro is built with `astro build` and `@astrojs/node` in standalone mode and served with `node dist/server/entry.mjs`; each live page computes in its frontmatter. The counter is a React island with `client:load`. All three prerender the landing and counter pages. Leptos is built with `cargo leptos build --release` and served by its Axum binary; each live page computes in a server function read through a blocking resource. Every Leptos page renders per request, the landing and counter pages included. Dioxus is built with `dx bundle --platform web --fullstack --release` and served by the server binary it writes; each live page computes in a server function read through `use_server_future`. Every Dioxus page also renders per request.",
  "A cold build starts from empty output directories with dependencies already downloaded and `node_modules` installed: a fresh `CARGO_TARGET_DIR` and no rustc wrapper for FSR, Leptos and Dioxus, a fresh `distDir` for Next.js and fresh output, build and Vite cache directories for SvelteKit, Nuxt and Astro. A warm build touches one page and builds again.",
  "The deploy tree is what a server needs: for FSR the tree `fsr bundle` writes plus `config/` and the binary as built with debug information; for Next.js the `standalone` directory with its static files copied in; for SvelteKit the adapter's `build/` and `package.json`; for Nuxt `.output/`; for Astro `dist/`; for Leptos its `site/` directory and the binary; for Dioxus the `web/` directory `dx` writes, which holds the server binary and `public/`. Next.js, SvelteKit, Nuxt and Astro also need Node installed, listed apart.",
  "Bytes per page are what a browser fetches to show it: the document, its stylesheets, the WebAssembly it preloads and its scripts, following the static imports of module scripts. Modules reached only by a dynamic `import()` are listed apart as loaded later. Sizes are raw and gzipped at the default level.",
  "Each load cell holds one HTTP/1.1 connection per concurrent client and sends the next request when the last body is read, for a fixed number of seconds. Each repetition starts every server afresh and the applications alternate, their order flipping between repetitions. Every page is checked for its status, its text and its product count before it is loaded. Figures are medians across repetitions.",
];

const NATIVE_NOTE: &str = "Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.";

const WASM_NOTE: &str = "Leptos and Dioxus hydrate by loading the whole app compiled to WebAssembly, so a page costs a browser this on top of its JavaScript. The other five hydrate with JavaScript alone and load none.";

const CAVEATS: &[&str] = &[
  "The load client runs on the same machine as the server and takes CPU from it.",
  "FSR, Leptos and Dioxus serve on every core. Next.js, SvelteKit, Nuxt and Astro each serve from one Node process; running several behind a balancer is a deployment choice this arena does not make.",
];

fn read(dir: &Path, name: &str) -> Option<Value> {
  serde_json::from_str(&std::fs::read_to_string(dir.join(format!("{name}.json"))).ok()?).ok()
}

fn median(mut values: Vec<f64>) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let mid = values.len() / 2;
  Some(if values.len() % 2 == 1 { values[mid] } else { (values[mid - 1] + values[mid]) / 2.0 })
}

fn round3(v: f64) -> f64 {
  (v * 1000.0).round() / 1000.0
}

fn thousands(v: f64) -> String {
  let digits = format!("{:.0}", v);
  let mut out = String::new();
  for (i, c) in digits.chars().enumerate() {
    if i > 0 && (digits.len() - i) % 3 == 0 {
      out.push(',');
    }
    out.push(c);
  }
  out
}

fn kib(bytes: f64) -> String {
  if bytes >= 1_048_576.0 { format!("{:.1} MiB", bytes / 1_048_576.0) } else { format!("{:.1} KiB", bytes / 1024.0) }
}

/// How far from a row's best figure another still counts as best, as a fraction of the best.
const BEST_WITHIN: f64 = 0.05;

/// Whether a row's figures pick a best: there are two or more and they are not all the same.
fn ranked(present: &[f64]) -> bool {
  present.len() > 1 && present.iter().any(|v| *v != present[0])
}

fn app_name(id: &str) -> &str {
  apps::by_id(id).map_or(id, |a| a.name)
}

/// A table section in the site's format and the same table as Markdown.
struct Table {
  id: String,
  title: String,
  heads: Vec<String>,
  apps: Vec<String>,
  rows: Vec<(Vec<String>, Vec<Option<(f64, String)>>)>,
  higher_is_better: bool,
  notes: Vec<String>,
  /// The median, min and max behind a cell, by row and column, for cells that summarise repeated samples. A cell without one carries no `median` in the JSON.
  spread: BTreeMap<(usize, usize), Spread>,
  /// A second figure shown under a cell's own, by row and column: the page measured with its loader computing in Rust.
  under: BTreeMap<(usize, usize), Under>,
  /// What an empty cell says when it means the app has none of the thing, rather than that it was not measured. Such a cell is never ranked.
  none: Option<&'static str>,
}

struct Under {
  /// The figure it is ranked by, against the row's own figures.
  value: f64,
  display: String,
  spread: Option<Spread>,
}

#[derive(Clone, Copy)]
struct Spread {
  median: f64,
  min: f64,
  max: f64,
  samples: usize,
}

impl Spread {
  fn write(&self, out: &mut Value) {
    out["median"] = json!(round3(self.median));
    if self.samples > 1 {
      out["min"] = json!(round3(self.min));
      out["max"] = json!(round3(self.max));
    }
  }

  fn of(values: &[f64]) -> Option<Spread> {
    let median = median(values.to_vec())?;
    let min = values.iter().cloned().fold(f64::MAX, f64::min);
    let max = values.iter().cloned().fold(f64::MIN, f64::max);
    Some(Spread { median, min, max, samples: values.len() })
  }
}

impl Table {
  /// Every figure in row `r` that competes for best, a native figure under a cell included, and the best of them.
  fn row_best(&self, r: usize, cells: &[Option<(f64, String)>]) -> (Vec<f64>, f64) {
    let present: Vec<f64> = cells.iter().flatten().map(|(v, _)| *v).chain(self.under.iter().filter(|((row, _), _)| *row == r).map(|(_, u)| u.value)).collect();
    let best = if self.higher_is_better { present.iter().cloned().fold(f64::MIN, f64::max) } else { present.iter().cloned().fold(f64::MAX, f64::min) };
    (present, best)
  }

  /// Whether `v` is marked best in a row of `present` figures whose best is `best`: the best itself or within [`BEST_WITHIN`] of it.
  fn is_best(&self, present: &[f64], best: f64, v: f64) -> bool {
    ranked(present) && if self.higher_is_better { v >= best * (1.0 - BEST_WITHIN) } else { v <= best * (1.0 + BEST_WITHIN) }
  }

  fn json(&self) -> Value {
    let rows: Vec<Value> = self
      .rows
      .iter()
      .enumerate()
      .map(|(r, (head, cells))| {
        let (present, best) = self.row_best(r, cells);
        let cells: Vec<Value> = cells
          .iter()
          .enumerate()
          .map(|(c, cell)| {
            cell.as_ref().map_or_else(|| self.none.map_or(Value::Null, |none| json!({ "display": none, "best": false, "above_ranked": false })), |(v, display)| {
              let mut out = json!({ "display": display, "best": self.is_best(&present, best, *v), "above_ranked": false });
              if let Some(spread) = self.spread.get(&(r, c)) {
                spread.write(&mut out);
              }
              if let Some(under) = self.under.get(&(r, c)) {
                let mut native = json!({ "display": under.display, "best": self.is_best(&present, best, under.value) });
                if let Some(spread) = &under.spread {
                  spread.write(&mut native);
                }
                out["native"] = native;
              }
              out
            })
          })
          .collect();
        json!({ "head": head, "cells": cells })
      })
      .collect();
    json!({ "id": self.id, "title": self.title, "mode": Value::Null, "batched": false, "heads": self.heads, "libraries": self.apps, "rows": rows, "notes": self.notes })
  }

  fn markdown(&self) -> String {
    let mut out = format!("#### {}\n\n|", self.title);
    for h in &self.heads {
      let _ = write!(out, " {h} |");
    }
    for a in &self.apps {
      let _ = write!(out, " {} |", app_name(a));
    }
    out.push_str("\n|");
    for _ in &self.heads {
      out.push_str(" :--- |");
    }
    for _ in &self.apps {
      out.push_str(" ---: |");
    }
    out.push('\n');
    for (r, (head, cells)) in self.rows.iter().enumerate() {
      out.push('|');
      for h in head {
        let _ = write!(out, " {h} |");
      }
      let (present, best) = self.row_best(r, cells);
      for (c, cell) in cells.iter().enumerate() {
        let under = self.under.get(&(r, c)).map_or(String::new(), |u| if self.is_best(&present, best, u.value) { format!("<br>**native {}**", u.display) } else { format!("<br>native {}", u.display) });
        match cell {
          Some((v, display)) if self.is_best(&present, best, *v) => {
            let _ = write!(out, " **{display}**{under} |");
          }
          Some((_, display)) => {
            let _ = write!(out, " {display}{under} |");
          }
          None => {
            let _ = write!(out, " {}{under} |", self.none.unwrap_or("-"));
          }
        }
      }
      out.push('\n');
    }
    for note in &self.notes {
      let _ = write!(out, "\n{note}\n");
    }
    out.push('\n');
    out
  }
}

struct Group {
  id: &'static str,
  title: &'static str,
  blurb: &'static str,
  tables: Vec<Table>,
}

fn app_ids(raw: &Value) -> Vec<String> {
  let present: Vec<String> = raw["records"].as_array().map_or(Vec::new(), |rs| rs.iter().filter_map(|r| r["app"].as_str().map(str::to_owned)).collect());
  APPS.iter().map(|a| a.id.to_owned()).filter(|id| present.contains(id)).collect()
}

fn perf_groups(perf: &Value) -> (Vec<Group>, Vec<Value>) {
  let apps = app_ids(perf);
  let records = perf["records"].as_array().cloned().unwrap_or_default();
  let levels: Vec<u64> = perf["levels"].as_array().map_or(Vec::new(), |l| l.iter().filter_map(Value::as_u64).collect());
  let cell = |app: &str, page: &str, c: u64, field: &str| -> Option<f64> {
    median(records.iter().filter(|r| r["app"] == app && r["page"] == page && r["c"] == c).filter_map(|r| r[field].as_f64()).collect())
  };
  let samples = |app: &str, page: &str, c: u64, field: &str| -> Vec<f64> { records.iter().filter(|r| r["app"] == app && r["page"] == page && r["c"] == c).filter_map(|r| r[field].as_f64()).collect() };
  let pages: Vec<&str> = PAGES.iter().map(|p| p.id).filter(|id| records.iter().any(|r| r["page"] == *id)).collect();

  let mut throughput = Vec::new();
  let mut latency = Vec::new();
  let mut cpu = Vec::new();
  let mut explore = Vec::new();
  for page in &pages {
    let blurb = crate::pages::by_id(page).map_or("", |p| p.blurb);
    let path = crate::pages::by_id(page).map_or("", |p| p.path);
    let native = format!("{page}{}", crate::pages::NATIVE);
    let mut spread = BTreeMap::new();
    let (mut rps_under, mut latency_under, mut cpu_under) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
    for (r, c) in levels.iter().enumerate() {
      for (col, a) in apps.iter().enumerate() {
        if let Some(found) = Spread::of(&samples(a, page, *c, "rps")) {
          spread.insert((r, col), found);
        }
        if let Some(found) = Spread::of(&samples(a, &native, *c, "rps")) {
          rps_under.insert((r, col), Under { value: found.median.round(), display: thousands(found.median), spread: Some(found) });
        }
        if let Some((p50, p99)) = cell(a, &native, *c, "p50_ms").zip(cell(a, &native, *c, "p99_ms")) {
          latency_under.insert((r, col), Under { value: p99, display: format!("{p50:.2} / {p99:.2}"), spread: None });
        }
        if let Some((cpu, rss)) = cell(a, &native, *c, "cpu_pct").zip(cell(a, &native, *c, "rss_mb")) {
          cpu_under.insert((r, col), Under { value: rss, display: format!("{cpu:.0}% / {rss:.1} MB"), spread: None });
        }
      }
    }
    let native_note = |notes: Vec<String>, under: &BTreeMap<(usize, usize), Under>| -> Vec<String> { if under.is_empty() { notes } else { notes.into_iter().chain([NATIVE_NOTE.to_owned()]).collect() } };
    throughput.push(Table {
      id: format!("rps-{page}"),
      title: format!("{page}: `{path}`"),
      heads: vec!["Clients".to_owned()],
      apps: apps.clone(),
      rows: levels.iter().map(|c| (vec![c.to_string()], apps.iter().map(|a| cell(a, page, *c, "rps").map(|v| (v.round(), thousands(v)))).collect())).collect(),
      higher_is_better: true,
      notes: native_note(vec![blurb.to_owned()], &rps_under),
      spread,
      under: rps_under,
      none: None,
    });
    latency.push(Table {
      id: format!("latency-{page}"),
      title: format!("{page}: p50 / p99 ms"),
      heads: vec!["Clients".to_owned()],
      apps: apps.clone(),
      rows: levels
        .iter()
        .map(|c| (vec![c.to_string()], apps.iter().map(|a| cell(a, page, *c, "p50_ms").zip(cell(a, page, *c, "p99_ms")).map(|(p50, p99)| (p99, format!("{p50:.2} / {p99:.2}")))).collect()))
        .collect(),
      higher_is_better: false,
      notes: native_note(vec!["Ranked on p99.".to_owned()], &latency_under),
      spread: BTreeMap::new(),
      under: latency_under,
      none: None,
    });
    cpu.push(Table {
      id: format!("cpu-{page}"),
      title: format!("{page}: server CPU % and resident MB"),
      heads: vec!["Clients".to_owned()],
      apps: apps.clone(),
      rows: levels
        .iter()
        .map(|c| (vec![c.to_string()], apps.iter().map(|a| cell(a, page, *c, "cpu_pct").zip(cell(a, page, *c, "rss_mb")).map(|(cpu, rss)| (rss, format!("{cpu:.0}% / {rss:.1} MB")))).collect()))
        .collect(),
      higher_is_better: false,
      notes: native_note(vec!["CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.".to_owned()], &cpu_under),
      spread: BTreeMap::new(),
      under: cpu_under,
      none: None,
    });
    for c in &levels {
      for a in &apps {
        if let Some(found) = Spread::of(&samples(a, page, *c, "rps")) {
          let mut record = json!({ "library": a, "d": [page, c.to_string()], "median": found.median.round() });
          if found.samples > 1 {
            record["min"] = json!(found.min.round());
            record["max"] = json!(found.max.round());
          }
          if let Some(p99) = cell(a, page, *c, "p99_ms") {
            record["note"] = json!(format!("p99 {p99:.2} ms"));
          }
          if let Some(native) = Spread::of(&samples(a, &native, *c, "rps")) {
            let mut out = json!({});
            native.write(&mut out);
            record["native"] = out;
          }
          explore.push(record);
        }
      }
    }
  }

  let servers = perf["servers"].as_array().cloned().unwrap_or_default();
  let server = |app: &str, field: &str| median(servers.iter().filter(|s| s["app"] == app).filter_map(|s| s[field].as_f64()).collect());
  let start = Table {
    id: "server".to_owned(),
    title: "Starting and idling".to_owned(),
    heads: vec!["Measure".to_owned()],
    apps: apps.clone(),
    rows: vec![
      (vec!["Spawn to first 200".to_owned()], apps.iter().map(|a| server(a, "cold_ms").map(|v| (v, format!("{v:.0} ms")))).collect()),
      (vec!["Resident at first 200".to_owned()], apps.iter().map(|a| server(a, "idle_rss_mb").map(|v| (v, format!("{v:.1} MB")))).collect()),
      (vec!["Peak resident over the run".to_owned()], apps.iter().map(|a| server(a, "peak_rss_mb").map(|v| (v, format!("{v:.1} MB")))).collect()),
    ],
    higher_is_better: false,
    notes: vec![],
    spread: BTreeMap::new(),
    under: BTreeMap::new(),
    none: None,
  };

  let groups = vec![
    Group { id: "throughput", title: "Requests per second", blurb: "Responses read in full per second at each number of concurrent clients.", tables: throughput },
    Group { id: "latency", title: "Latency", blurb: "From sending a request to having read its body, median and 99th percentile.", tables: latency },
    Group { id: "resources", title: "CPU and memory", blurb: "What the server used while it was loaded and what it takes to start.", tables: std::iter::once(start).chain(cpu).collect() },
  ];
  (groups, explore)
}

fn build_group(build: &Value) -> Group {
  let apps = app_ids(build);
  let records = build["records"].as_array().cloned().unwrap_or_default();
  let secs = |app: &str, kind: &str| -> Vec<f64> { records.iter().filter(|r| r["app"] == app && r["kind"] == kind).filter_map(|r| r["secs"].as_f64()).collect() };
  let kinds = ["cold", "warm"];
  let mut spread = BTreeMap::new();
  for (r, kind) in kinds.iter().enumerate() {
    for (c, a) in apps.iter().enumerate() {
      if let Some(found) = Spread::of(&secs(a, kind)) {
        spread.insert((r, c), found);
      }
    }
  }
  let rows = kinds
    .iter()
    .map(|kind| (vec![(*kind).to_owned()], apps.iter().map(|a| median(secs(a, kind)).map(|v| (v, format!("{v:.1} s")))).collect()))
    .collect();
  Group {
    id: "build",
    title: "Build",
    blurb: "Wall time of a production build, from nothing compiled and after one page changed.",
    tables: vec![Table { id: "build".to_owned(), title: "Production build".to_owned(), heads: vec!["Build".to_owned()], apps, rows, higher_is_better: false, notes: vec![], spread, under: BTreeMap::new(), none: None }],
  }
}

fn size_group(size: &Value) -> Group {
  let apps = app_ids(size);
  let records = size["records"].as_array().cloned().unwrap_or_default();
  let of = |app: &str| records.iter().find(|r| r["app"] == app).cloned().unwrap_or(Value::Null);
  let bytes = |v: &Value| v.as_f64().map(|b| (b, kib(b)));
  let deploy = Table {
    id: "deploy".to_owned(),
    title: "Deploy".to_owned(),
    heads: vec!["Measure".to_owned()],
    apps: apps.clone(),
    rows: vec![
      (vec!["Deploy tree".to_owned()], apps.iter().map(|a| bytes(&of(a)["deploy"]["tree_bytes"])).collect()),
      (vec!["Files in the tree".to_owned()], apps.iter().map(|a| of(a)["deploy"]["tree_files"].as_f64().map(|f| (f, thousands(f)))).collect()),
      (vec!["Server binary as built".to_owned()], apps.iter().map(|a| bytes(&of(a)["deploy"]["binary_bytes"])).collect()),
      (vec!["Server binary stripped".to_owned()], apps.iter().map(|a| bytes(&of(a)["deploy"]["stripped_binary_bytes"])).collect()),
      (
        vec!["Runtime needed beside it".to_owned()],
        apps
          .iter()
          .map(|a| {
            let d = &of(a)["deploy"];
            d["runtime"].as_str().zip(d["runtime_bytes"].as_f64()).map(|(name, b)| (b, format!("{name}, {}", kib(b))))
          })
          .collect(),
      ),
      (
        vec!["Total to deploy".to_owned()],
        apps
          .iter()
          .map(|a| {
            let d = &of(a)["deploy"];
            d["tree_bytes"].as_f64().map(|tree| tree + d["runtime_bytes"].as_f64().unwrap_or(0.0)).map(|b| (b, kib(b)))
          })
          .collect(),
      ),
    ],
    higher_is_better: false,
    notes: vec!["The total is the deploy tree, which holds the server binary where there is one, plus the runtime that has to be installed beside it.".to_owned()],
    spread: BTreeMap::new(),
    under: BTreeMap::new(),
    none: None,
  };
  let page_ids: Vec<&str> = PAGES.iter().map(|p| p.id).filter(|id| records.iter().any(|r| !r["pages"][*id].is_null())).collect();
  let field_table = |id: &str, title: &str, raw: &str, gz: &str, none: Option<&'static str>, note: &str| Table {
    id: id.to_owned(),
    title: title.to_owned(),
    heads: vec!["Page".to_owned()],
    apps: apps.clone(),
    rows: page_ids
      .iter()
      .map(|p| {
        (
          vec![(*p).to_owned()],
          apps.iter().map(|a| { let page = &of(a)["pages"][*p]; page[gz].as_f64().zip(page[raw].as_f64()).filter(|(_, r)| none.is_none() || *r > 0.0).map(|(g, r)| (g, format!("{} ({} raw)", kib(g), kib(r)))) }).collect(),
        )
      })
      .collect(),
    higher_is_better: false,
    notes: std::iter::once("Gzipped, with the raw size beside it.").chain((!note.is_empty()).then_some(note)).map(str::to_owned).collect(),
    spread: BTreeMap::new(),
    under: BTreeMap::new(),
    none,
  };
  let requests = Table {
    id: "requests".to_owned(),
    title: "Requests to show the page".to_owned(),
    heads: vec!["Page".to_owned()],
    apps: apps.clone(),
    rows: page_ids
      .iter()
      .map(|p| {
        (
          vec![(*p).to_owned()],
          apps
            .iter()
            .map(|a| {
              let page = &of(a)["pages"][*p];
              page["requests"].as_f64().map(|n| {
                let later = page["later_requests"].as_f64().unwrap_or(0.0);
                (n, if later > 0.0 { format!("{n:.0}, then {later:.0} later ({})", kib(page["later_js_gz"].as_f64().unwrap_or(0.0))) } else { format!("{n:.0}") })
              })
            })
            .collect(),
        )
      })
      .collect(),
    higher_is_better: false,
    notes: vec!["The document, stylesheets, WebAssembly and scripts a browser fetches before the page is whole. Modules reached only through a dynamic `import()` follow as loaded later, gzipped.".to_owned()],
    spread: BTreeMap::new(),
    under: BTreeMap::new(),
    none: None,
  };
  Group {
    id: "size",
    title: "Size",
    blurb: "What a deployment carries and what each page costs a browser.",
    tables: vec![deploy, field_table("html", "Document", "html", "html_gz", None, ""), field_table("js", "JavaScript", "js", "js_gz", None, ""), field_table("wasm", "WebAssembly", "wasm", "wasm_gz", Some("-"), WASM_NOTE), field_table("css", "CSS", "css", "css_gz", None, ""), requests],
  }
}

/// What a run that has not measured every cell yet is missing, by app and repetition, or nothing when it is whole.
fn partial(perf: &Value) -> Option<String> {
  let records = perf["records"].as_array().cloned().unwrap_or_default();
  let levels = perf["levels"].as_array().map_or(0, Vec::len);
  let reps = perf["reps"].as_u64().unwrap_or(0) as usize;
  let mut missing = Vec::new();
  let mut wanted_all = 0;
  for app in APPS {
    let per_rep = PAGES.iter().map(|p| p.variants(app.native()).len()).sum::<usize>() * levels;
    wanted_all += per_rep * reps;
    for rep in 0..reps {
      let have = records.iter().filter(|r| r["app"] == app.id && r["rep"].as_u64() == Some(rep as u64)).count();
      if have < per_rep {
        missing.push(format!("{} repetition {} has {have} of {per_rep}", app.name, rep + 1));
      }
    }
  }
  (records.len() < wanted_all).then(|| format!("Partial: {} of {wanted_all} cells measured, so a figure may be the median of fewer than {reps} repetitions. {}.", records.len(), missing.join(", ")))
}

/// Each app's versions with what each measured, from `[perf, build, size]`: the pages of the load cells, the build and the sizes. A record without a version of its own has the one its file recorded.
fn measured_versions(sources: &[Option<&Value>]) -> BTreeMap<String, String> {
  let mut seen: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
  for (at, source) in sources.iter().enumerate() {
    let Some(source) = source else { continue };
    for record in source["records"].as_array().into_iter().flatten() {
      let app = record["app"].as_str().unwrap_or("?").to_owned();
      let version = record["version"].as_str().or_else(|| source["versions"][&app].as_str()).unwrap_or("?").to_owned();
      let what = match at {
        0 => record["page"].as_str().unwrap_or("?").to_owned(),
        1 => "the build".to_owned(),
        _ => "the sizes".to_owned(),
      };
      let versions = seen.entry(app).or_default();
      let at = match versions.iter().position(|(v, _)| *v == version) {
        Some(at) => at,
        None => {
          versions.push((version, Vec::new()));
          versions.len() - 1
        }
      };
      if !versions[at].1.contains(&what) {
        versions[at].1.push(what);
      }
    }
  }
  seen
    .into_iter()
    .map(|(app, versions)| {
      let line = match versions.as_slice() {
        [(version, _)] => version.clone(),
        _ => versions.iter().map(|(version, what)| format!("{version} for {}", and_list(what))).collect::<Vec<_>>().join("; "),
      };
      (app, line)
    })
    .collect()
}

fn and_list(items: &[String]) -> String {
  match items {
    [] => String::new(),
    [one] => one.clone(),
    [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
  }
}

/// When the builds ran, with the apps each sitting covers when they were not all built at once.
fn build_times(build: &Value) -> String {
  let mut sittings: Vec<(String, Vec<String>)> = Vec::new();
  for record in build["records"].as_array().into_iter().flatten() {
    let start = record.get("start").unwrap_or(&build["start"]);
    let end = record.get("end").unwrap_or(&build["end"]);
    let line = format!("{}. {}", state_line("Started", start), state_line("ended", end));
    let app = app_name(record["app"].as_str().unwrap_or("?")).to_owned();
    match sittings.iter_mut().find(|(l, _)| *l == line) {
      Some((_, apps)) => {
        if !apps.contains(&app) {
          apps.push(app);
        }
      }
      None => sittings.push((line, vec![app])),
    }
  }
  match sittings.as_slice() {
    [(line, _)] => line.clone(),
    _ => sittings.iter().map(|(line, apps)| format!("{line} for {}", and_list(apps))).collect::<Vec<_>>().join("; "),
  }
}

/// When the sizes were measured, with the apps each time covers when they were not all measured at once.
fn size_times(size: &Value) -> String {
  let mut times: Vec<(String, Vec<String>)> = Vec::new();
  for record in size["records"].as_array().into_iter().flatten() {
    let at = record["at"].as_str().or_else(|| size["at"].as_str()).unwrap_or("?").to_owned();
    let app = app_name(record["app"].as_str().unwrap_or("?")).to_owned();
    match times.iter_mut().find(|(t, _)| *t == at) {
      Some((_, apps)) => apps.push(app),
      None => times.push((at, vec![app])),
    }
  }
  match times.as_slice() {
    [(at, _)] => at.clone(),
    _ => times.iter().map(|(at, apps)| format!("{at} for {}", and_list(apps))).collect::<Vec<_>>().join("; "),
  }
}

fn state_line(label: &str, state: &Value) -> String {
  format!("{label} {} in {} power mode at load {}", state["at"].as_str().unwrap_or("?"), state["power_mode"].as_str().unwrap_or("?"), state["load"])
}

pub fn write(raw: &Path, docs: &Path) -> Result<Vec<PathBuf>, String> {
  let perf = read(raw, "perf");
  let build = read(raw, "build");
  let size = read(raw, "size");
  if perf.is_none() && build.is_none() && size.is_none() {
    return Err(format!("nothing under {}: run build, size or run first", raw.display()));
  }

  let mut groups = Vec::new();
  let mut explore = Vec::new();
  if let Some(perf) = &perf {
    let (g, e) = perf_groups(perf);
    groups.extend(g);
    explore = e;
  }
  if let Some(build) = &build {
    groups.push(build_group(build));
  }
  if let Some(size) = &size {
    groups.push(size_group(size));
  }

  let versions = measured_versions(&[perf.as_ref(), build.as_ref(), size.as_ref()]);
  let machine = [&perf, &build, &size].into_iter().flatten().find_map(|s| s["machine"].as_str().map(str::to_owned)).unwrap_or_default();

  let mut md = String::from("# Arena results\n\n");
  let _ = writeln!(md, "{SUMMARY}\n\n{READING}\n");
  let _ = writeln!(md, "Machine: {machine}.\n");
  for (id, version) in &versions {
    let _ = writeln!(md, "- {}: {version}", app_name(id));
  }
  md.push('\n');
  if let Some(perf) = &perf {
    let _ = writeln!(md, "Load: {} s cells after {} s of warm-up per page, {} client threads. {}. {}.\n", perf["secs"], perf["warmup_secs"], perf["client_threads"], state_line("Started", &perf["start"]), state_line("ended", &perf["end"]));
    if let Some(partial) = partial(perf) {
      let _ = writeln!(md, "{partial}\n");
    }
  }
  if let Some(build) = &build {
    let _ = writeln!(md, "Build: {}.\n", build_times(build));
  }
  if let Some(size) = &size {
    let _ = writeln!(md, "Size: measured {}.\n", size_times(size));
  }
  for group in &groups {
    let _ = writeln!(md, "## {}\n\n{}\n", group.title, group.blurb);
    for table in &group.tables {
      md.push_str(&table.markdown());
    }
  }
  md.push_str("## Method\n\n");
  for line in METHOD {
    let _ = writeln!(md, "- {line}");
  }
  md.push_str("\n## Caveats\n\n");
  for line in CAVEATS {
    let _ = writeln!(md, "- {line}");
  }

  let levels: Vec<String> = perf.as_ref().and_then(|p| p["levels"].as_array().cloned()).unwrap_or_default().iter().map(|v| v.to_string()).collect();
  let page_values: Vec<&str> = PAGES.iter().map(|p| p.id).filter(|id| explore.iter().any(|r| r["d"][0] == *id)).collect();
  let site = json!({
    "format": 1,
    "id": "web_ssr_arena",
    "title": "Arena",
    "summary": SUMMARY,
    "noun": "framework",
    "peers": "other server-rendering web frameworks",
    "reading": READING,
    "unit": { "label": "req/s", "description": "Responses read in full per second." },
    "run": {
      "machine": machine,
      "measured_at": perf.as_ref().and_then(|p| p["start"]["at"].as_str().map(str::to_owned)),
      "power_mode": perf.as_ref().and_then(|p| p["start"]["power_mode"].as_str().map(str::to_owned)),
      "load_at_start": perf.as_ref().map(|p| p["start"]["load"].clone()),
      "load_at_end": perf.as_ref().map(|p| p["end"]["load"].clone()),
      "secs": perf.as_ref().map(|p| p["secs"].clone()),
    },
    "libraries": versions.iter().map(|(id, v)| json!({ "name": id, "crate": app_name(id), "version": v })).collect::<Vec<_>>(),
    "method": METHOD,
    "caveats": CAVEATS,
    "groups": groups.iter().map(|g| json!({ "id": g.id, "title": g.title, "blurb": g.blurb, "caveats": [], "sections": g.tables.iter().map(Table::json).collect::<Vec<_>>() })).collect::<Vec<_>>(),
    "explore": {
      "intro": "Choose a page and compare the frameworks at each number of concurrent clients. Bars show requests per second, the median across repetitions, with the 99th percentile latency beside it.",
      "axes": "pages and concurrent clients",
      "rows": "clients",
      "defaults": { "page": "catalog" },
      "dimensions": [
        { "id": "page", "label": "Page", "values": page_values },
        { "id": "clients", "label": "Clients", "values": levels },
      ],
      "records": explore,
    },
  });

  std::fs::create_dir_all(docs.join("raw")).map_err(|e| e.to_string())?;
  let readme = docs.join("README.md");
  std::fs::write(&readme, md).map_err(|e| format!("{}: {e}", readme.display()))?;
  let results = docs.join("raw/results.json");
  let mut text = serde_json::to_string_pretty(&site).map_err(|e| e.to_string())?;
  text.push('\n');
  std::fs::write(&results, text).map_err(|e| format!("{}: {e}", results.display()))?;
  Ok(vec![readme, results])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn each_version_is_named_with_what_it_measured() {
    let perf = json!({ "versions": { "fsr": "B" }, "records": [
      { "app": "fsr", "page": "home", "version": "A" },
      { "app": "fsr", "page": "api", "version": "B" },
      { "app": "fsr", "page": "counter" },
      { "app": "fsr", "page": "home", "version": "A" },
      { "app": "leptos", "page": "home", "version": "L" },
    ]});
    let build = json!({ "versions": { "fsr": "A", "leptos": "L" }, "records": [{ "app": "fsr" }, { "app": "leptos" }] });
    let lines = measured_versions(&[Some(&perf), Some(&build), None]);
    assert_eq!(lines["fsr"], "A for home and the build; B for api and counter");
    assert_eq!(lines["leptos"], "L");
  }

  #[test]
  fn a_native_figure_is_written_under_the_cell_it_belongs_to() {
    let spread = Spread { median: 74425.0, min: 74000.0, max: 75000.0, samples: 2 };
    let table = Table {
      id: "rps-primes".to_owned(),
      title: "primes".to_owned(),
      heads: vec!["Clients".to_owned()],
      apps: vec!["fsr".to_owned(), "leptos".to_owned()],
      rows: vec![(vec!["128".to_owned()], vec![Some((894.0, "894".to_owned())), Some((74425.0, "74,425".to_owned()))])],
      higher_is_better: true,
      notes: vec![],
      spread: BTreeMap::new(),
      under: BTreeMap::from([((0, 0), Under { value: 70000.0, display: "70,000".to_owned(), spread: Some(spread) })]),
      none: None,
    };
    assert!(table.markdown().contains("| 128 | 894<br>native 70,000 | **74,425** |"));
    let json = table.json();
    assert_eq!(json["rows"][0]["cells"][0]["native"]["display"], "70,000");
    assert_eq!(json["rows"][0]["cells"][0]["native"]["median"], 74425.0);
    assert_eq!(json["rows"][0]["cells"][0]["best"], false);
    assert!(json["rows"][0]["cells"][1].get("native").is_none());
  }

  #[test]
  fn an_app_with_none_says_so_and_is_not_ranked() {
    let table = Table {
      id: "wasm".to_owned(),
      title: "WebAssembly".to_owned(),
      heads: vec!["Page".to_owned()],
      apps: vec!["fsr".to_owned(), "leptos".to_owned(), "dioxus".to_owned()],
      rows: vec![(vec!["home".to_owned()], vec![None, Some((263270.0, "257.1 KiB".to_owned())), Some((494797.0, "483.2 KiB".to_owned()))])],
      higher_is_better: false,
      notes: vec![],
      spread: BTreeMap::new(),
      under: BTreeMap::new(),
      none: Some("-"),
    };
    assert!(table.markdown().contains("| home | - | **257.1 KiB** | 483.2 KiB |"));
    assert_eq!(table.json()["rows"][0]["cells"][0], json!({ "display": "-", "best": false, "above_ranked": false }));
  }

  #[test]
  fn a_build_of_one_app_says_when_each_was_built() {
    let state = |at: &str| json!({ "at": at, "power_mode": "high power", "load": 5.0 });
    let build = json!({ "records": [
      { "app": "fsr", "start": state("t2"), "end": state("t3") },
      { "app": "leptos", "start": state("t0"), "end": state("t1") },
      { "app": "dioxus", "start": state("t0"), "end": state("t1") },
    ]});
    assert_eq!(
      build_times(&build),
      "Started t2 in high power power mode at load 5.0. ended t3 in high power power mode at load 5.0 for SnapFire FSR; Started t0 in high power power mode at load 5.0. ended t1 in high power power mode at load 5.0 for Leptos and Dioxus"
    );
    let old = json!({ "start": state("t0"), "end": state("t1"), "records": [{ "app": "fsr" }] });
    assert_eq!(build_times(&old), "Started t0 in high power power mode at load 5.0. ended t1 in high power power mode at load 5.0");
  }

  #[test]
  fn every_figure_within_five_percent_of_the_best_is_marked_best() {
    let row = |higher_is_better: bool, values: &[f64]| Table {
      id: "t".to_owned(),
      title: "t".to_owned(),
      heads: vec!["Clients".to_owned()],
      apps: values.iter().map(|_| "fsr".to_owned()).collect(),
      rows: vec![(vec!["128".to_owned()], values.iter().map(|v| Some((*v, v.to_string()))).collect())],
      higher_is_better,
      notes: vec![],
      spread: BTreeMap::new(),
      under: BTreeMap::new(),
      none: None,
    };
    let best = |table: Table| table.json()["rows"][0]["cells"].as_array().unwrap().iter().map(|c| c["best"] == true).collect::<Vec<_>>();
    assert_eq!(best(row(true, &[164459.0, 6717.0, 39999.0, 169373.0, 170573.0, 162000.0])), [true, false, false, true, true, false]);
    assert_eq!(best(row(false, &[1.0, 1.05, 1.06])), [true, true, false]);
    assert!(row(true, &[100.0, 96.0, 90.0]).markdown().contains("| 128 | **100** | **96** | 90 |"));
  }

  #[test]
  fn a_row_whose_figures_are_all_the_same_has_no_best() {
    assert!(!ranked(&[107.3, 107.3, 107.3]));
    assert!(!ranked(&[6.0]));
    assert!(ranked(&[6.0, 6.0, 181.0]));
  }

  #[test]
  fn a_run_missing_cells_says_which() {
    let perf = json!({ "levels": [1], "reps": 1, "records": [{ "app": "fsr", "rep": 0 }] });
    let said = partial(&perf).unwrap();
    assert!(said.starts_with("Partial: 1 of "), "{said}");
    assert!(said.contains("SnapFire FSR repetition 1 has 1 of 14"), "{said}");
  }

  #[test]
  fn a_native_figure_that_beats_the_row_is_the_best() {
    let table = Table {
      id: "rps-report".to_owned(),
      title: "report".to_owned(),
      heads: vec!["Clients".to_owned()],
      apps: vec!["fsr".to_owned(), "leptos".to_owned()],
      rows: vec![(vec!["128".to_owned()], vec![Some((4818.0, "4,818".to_owned())), Some((91292.0, "91,292".to_owned()))])],
      higher_is_better: true,
      notes: vec![],
      spread: BTreeMap::new(),
      under: BTreeMap::from([((0, 0), Under { value: 98020.0, display: "98,020".to_owned(), spread: None })]),
      none: None,
    };
    assert!(table.markdown().contains("| 128 | 4,818<br>**native 98,020** | 91,292 |"));
    let json = table.json();
    assert_eq!(json["rows"][0]["cells"][0]["native"]["best"], true);
    assert_eq!(json["rows"][0]["cells"][0]["best"], false);
    assert_eq!(json["rows"][0]["cells"][1]["best"], false);
  }
}
