//! The applications under test: how each is built, served and packed for deployment.

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
  Fsr,
  Next,
  SvelteKit,
  Nuxt,
  Leptos,
  Dioxus,
}

pub struct App {
  pub id: &'static str,
  pub name: &'static str,
  pub kind: Kind,
  pub port: u16,
}

pub const APPS: &[App] = &[
  App { id: "fsr", name: "SnapFire FSR", kind: Kind::Fsr, port: 3101 },
  App { id: "nextjs", name: "Next.js", kind: Kind::Next, port: 3102 },
  App { id: "sveltekit", name: "SvelteKit", kind: Kind::SvelteKit, port: 3103 },
  App { id: "nuxt", name: "Nuxt", kind: Kind::Nuxt, port: 3104 },
  App { id: "leptos", name: "Leptos", kind: Kind::Leptos, port: 3105 },
  App { id: "dioxus", name: "Dioxus", kind: Kind::Dioxus, port: 3106 },
];

pub fn by_id(id: &str) -> Option<&'static App> {
  APPS.iter().find(|a| a.id == id)
}

pub fn root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// One command of a build, run in `dir`.
pub struct Step {
  pub label: &'static str,
  pub command: Command,
}

const FSR_BIN: &str = "outfitters_fsr";

const LEPTOS_BIN: &str = "outfitters_leptos";

/// Where `dx bundle` writes the Dioxus server and its `public/` under a target directory.
const DIOXUS_WEB: &str = "dx/outfitters_dioxus/release/web";

/// Next requires its dist directory inside the project.
const COLD_NEXT: &str = ".next-cold";

/// Set for a cold build of SvelteKit or Nuxt, whose configs then write every output and cache to directories ending in `-cold`.
const ARENA_COLD: &str = "ARENA_COLD";

/// The key the FSR host signs sessions with during a run; the host refuses to start on the placeholder in `config/production.toml`.
const SESSION_KEY: &str = "5f1d3c0b9a8e7d6c5b4a39281706f5e4d3c2b1a09f8e7d6c5b4a392817060504";

impl App {
  pub fn dir(&self) -> PathBuf {
    root().join("apps").join(self.id)
  }

  pub fn origin(&self) -> String {
    format!("http://127.0.0.1:{}", self.port)
  }

  /// The commands of a build. A cold one writes to directories nothing has built into ([`App::cold_dirs`]) and runs rustc with no wrapper, so no compiled dependency or build cache is reused.
  pub fn build(&self, cold: bool) -> Vec<Step> {
    let dir = self.dir();
    match self.kind {
      Kind::Fsr => {
        let target = if cold { self.cold_dirs().remove(0) } else { dir.join("target") };
        let mut emit = Command::new(fsr());
        emit.args(["build", "app"]).current_dir(&dir).env("RELEASE_ENV", "production");
        let mut cargo = Command::new("cargo");
        cargo.args(["build", "--release"]).current_dir(&dir).env("RELEASE_ENV", "production").env("CARGO_TARGET_DIR", &target);
        if cold {
          cargo.env("RUSTC_WRAPPER", "");
        }
        let mut prerender = Command::new(target.join("release").join(FSR_BIN));
        prerender.arg("prerender").current_dir(&dir).env("RELEASE_ENV", "production").env("C5_SESSION__KEY", SESSION_KEY);
        vec![Step { label: "fsr build app", command: emit }, Step { label: "cargo build --release", command: cargo }, Step { label: "prerender", command: prerender }]
      }
      Kind::Next => {
        let mut next = Command::new(dir.join("node_modules/.bin/next"));
        next.arg("build").current_dir(&dir).env("NEXT_TELEMETRY_DISABLED", "1");
        if cold {
          next.env("NEXT_DIST_DIR", COLD_NEXT);
        }
        vec![Step { label: "next build", command: next }]
      }
      Kind::SvelteKit => {
        let mut vite = Command::new(dir.join("node_modules/.bin/vite"));
        vite.arg("build").current_dir(&dir);
        if cold {
          vite.env(ARENA_COLD, "1");
        }
        vec![Step { label: "vite build", command: vite }]
      }
      Kind::Nuxt => {
        let mut nuxt = Command::new(dir.join("node_modules/.bin/nuxt"));
        nuxt.arg("build").current_dir(&dir).env("NUXT_TELEMETRY_DISABLED", "1");
        if cold {
          nuxt.env(ARENA_COLD, "1");
        }
        vec![Step { label: "nuxt build", command: nuxt }]
      }
      Kind::Leptos => {
        let mut leptos = Command::new("cargo");
        leptos.args(["leptos", "build", "--release"]).current_dir(&dir);
        if cold {
          let target = self.cold_dirs().remove(0);
          leptos.env("CARGO_TARGET_DIR", &target).env("LEPTOS_SITE_ROOT", target.join("site")).env("RUSTC_WRAPPER", "");
        }
        vec![Step { label: "cargo leptos build --release", command: leptos }]
      }
      Kind::Dioxus => {
        let mut dx = Command::new(dx());
        dx.args(["bundle", "--platform", "web", "--fullstack", "--release"]).current_dir(&dir).envs(dx_env());
        if cold {
          dx.env("CARGO_TARGET_DIR", self.cold_dirs().remove(0)).env("RUSTC_WRAPPER", "");
        }
        vec![Step { label: "dx bundle --release", command: dx }]
      }
    }
  }

  /// Where a cold build writes; the harness removes them before and after the build it times.
  pub fn cold_dirs(&self) -> Vec<PathBuf> {
    let dir = self.dir();
    match self.kind {
      Kind::Fsr => vec![root().join("runs/cold-fsr-target")],
      Kind::Next => vec![dir.join(COLD_NEXT)],
      Kind::SvelteKit => vec![dir.join(".svelte-kit-cold"), dir.join("build-cold"), dir.join(".vite-cold")],
      Kind::Nuxt => vec![dir.join(".nuxt-cold"), dir.join(".output-cold"), dir.join(".vite-cold")],
      Kind::Leptos => vec![root().join("runs/cold-leptos-target")],
      Kind::Dioxus => vec![root().join("runs/cold-dioxus-target")],
    }
  }

  /// What a build leaves for the server: Next's standalone output needs its static files copied in beside it.
  pub fn finish(&self) -> std::io::Result<()> {
    match self.kind {
      Kind::Fsr | Kind::SvelteKit | Kind::Nuxt | Kind::Leptos | Kind::Dioxus => Ok(()),
      Kind::Next => {
        let next = self.dir().join(".next");
        let into = next.join("standalone/.next/static");
        if into.exists() {
          std::fs::remove_dir_all(&into)?;
        }
        copy_tree(&next.join("static"), &into)
      }
    }
  }

  pub fn serve(&self) -> Command {
    let dir = self.dir();
    match self.kind {
      Kind::Fsr => {
        let mut host = Command::new(dir.join("target/release").join(FSR_BIN));
        host.current_dir(&dir).env("RELEASE_ENV", "production").env("C5_SESSION__KEY", SESSION_KEY);
        host
      }
      Kind::Next => {
        let standalone = dir.join(".next/standalone");
        let mut node = Command::new("node");
        node.arg("server.js").current_dir(&standalone).env("NODE_ENV", "production").env("PORT", self.port.to_string()).env("HOSTNAME", "127.0.0.1").env("NEXT_TELEMETRY_DISABLED", "1");
        node
      }
      Kind::SvelteKit | Kind::Nuxt => {
        let entry = if self.kind == Kind::SvelteKit { "build" } else { ".output/server/index.mjs" };
        let mut node = Command::new("node");
        node.arg(entry).current_dir(&dir).env("NODE_ENV", "production").env("PORT", self.port.to_string()).env("HOST", "127.0.0.1");
        node
      }
      Kind::Leptos => {
        let mut server = Command::new(dir.join("target/release").join(LEPTOS_BIN));
        server.current_dir(&dir).envs(leptos_env(&dir.join("target/site"), self.port));
        server
      }
      Kind::Dioxus => {
        let web = dir.join("target").join(DIOXUS_WEB);
        let mut server = Command::new(web.join("server"));
        server.current_dir(&web).env("IP", "127.0.0.1").env("PORT", self.port.to_string());
        server
      }
    }
  }

  /// Writes the tree a server is deployed from into `out`, which must not exist yet.
  pub fn deploy(&self, out: &Path) -> Result<(), String> {
    let dir = self.dir();
    match self.kind {
      Kind::Fsr => {
        let fsr = fsr();
        let status = Command::new(&fsr).args(["bundle", "app", "--out"]).arg(out).current_dir(&dir).status().map_err(|e| format!("{fsr}: {e}"))?;
        if !status.success() {
          return Err(format!("{fsr} bundle app: {status}"));
        }
        std::fs::copy(dir.join("target/release").join(FSR_BIN), out.join(FSR_BIN)).map_err(|e| format!("the binary: {e}"))?;
        copy_tree(&dir.join("config"), &out.join("config")).map_err(|e| format!("config/: {e}"))
      }
      Kind::Next => copy_tree(&dir.join(".next/standalone"), out).map_err(|e| format!(".next/standalone: {e}")),
      Kind::SvelteKit => {
        copy_tree(&dir.join("build"), &out.join("build")).map_err(|e| format!("build/: {e}"))?;
        std::fs::copy(dir.join("package.json"), out.join("package.json")).map(|_| ()).map_err(|e| format!("package.json: {e}"))
      }
      Kind::Nuxt => copy_tree(&dir.join(".output"), &out.join(".output")).map_err(|e| format!(".output/: {e}")),
      Kind::Leptos => {
        copy_tree(&dir.join("target/site"), &out.join("site")).map_err(|e| format!("target/site: {e}"))?;
        std::fs::copy(dir.join("target/release").join(LEPTOS_BIN), out.join(LEPTOS_BIN)).map(|_| ()).map_err(|e| format!("the binary: {e}"))
      }
      Kind::Dioxus => copy_tree(&dir.join("target").join(DIOXUS_WEB), out).map_err(|e| format!("{DIOXUS_WEB}: {e}")),
    }
  }

  /// The server binary inside a deploy tree, when the application ships one.
  pub fn binary(&self) -> Option<&'static str> {
    match self.kind {
      Kind::Fsr => Some(FSR_BIN),
      Kind::Leptos => Some(LEPTOS_BIN),
      Kind::Dioxus => Some("server"),
      Kind::Next | Kind::SvelteKit | Kind::Nuxt => None,
    }
  }

  /// The program a deploy tree needs installed beside it to run, with its version and size on disk.
  pub fn runtime(&self) -> Option<(String, u64)> {
    match self.kind {
      Kind::Fsr | Kind::Leptos | Kind::Dioxus => None,
      Kind::Next | Kind::SvelteKit | Kind::Nuxt => {
        let path = crate::machine::output("which", &["node"])?;
        let version = crate::machine::output("node", &["--version"])?;
        Some((format!("Node {version}"), std::fs::metadata(path).ok()?.len()))
      }
    }
  }

  /// The framework's version as this tree builds it.
  pub fn version(&self) -> String {
    match self.kind {
      Kind::Fsr => {
        let lock = std::fs::read_to_string(self.dir().join("Cargo.lock")).unwrap_or_default();
        let host = locked(&lock, "snapfire_fsr_host").unwrap_or_else(|| "?".to_owned());
        format!("snapfire_fsr_host {host}")
      }
      Kind::Leptos => {
        let lock = std::fs::read_to_string(self.dir().join("Cargo.lock")).unwrap_or_default();
        format!("leptos {}", locked(&lock, "leptos").unwrap_or_else(|| "?".to_owned()))
      }
      Kind::Dioxus => {
        let lock = std::fs::read_to_string(self.dir().join("Cargo.lock")).unwrap_or_default();
        format!("dioxus {}", locked(&lock, "dioxus").unwrap_or_else(|| "?".to_owned()))
      }
      Kind::Next | Kind::SvelteKit | Kind::Nuxt => {
        let read = |package: &str| -> String {
          let version = || -> Option<String> {
            let text = std::fs::read_to_string(self.dir().join("node_modules").join(package).join("package.json")).ok()?;
            let value: serde_json::Value = serde_json::from_str(&text).ok()?;
            value["version"].as_str().map(str::to_owned)
          };
          format!("{package} {}", version().unwrap_or_else(|| "?".to_owned()))
        };
        let (framework, view) = match self.kind {
          Kind::SvelteKit => ("@sveltejs/kit", "svelte"),
          Kind::Nuxt => ("nuxt", "vue"),
          _ => ("next", "react"),
        };
        format!("{}, {}", read(framework), read(view))
      }
    }
  }

  /// A source file a warm rebuild touches.
  pub fn page_source(&self) -> PathBuf {
    let dir = self.dir();
    match self.kind {
      Kind::Fsr => dir.join("app/routes/products/page.tsx"),
      Kind::Next => dir.join("app/products/page.tsx"),
      Kind::SvelteKit => dir.join("src/routes/products/+page.svelte"),
      Kind::Nuxt => dir.join("app/pages/products/index.vue"),
      Kind::Leptos => dir.join("src/app.rs"),
      Kind::Dioxus => dir.join("src/main.rs"),
    }
  }

  /// Each file this application keeps a copy of, by its path under `shared/` and its path in the application.
  /// Whether a loader can call Rust, so the compute pages are also measured computing in Rust.
  pub fn native(&self) -> bool {
    self.kind == Kind::Fsr
  }

  pub fn shared_copies(&self) -> Vec<(String, String)> {
    let pair = |shared: &str, copy: &str| (shared.to_owned(), copy.to_owned());
    if self.kind == Kind::Leptos {
      return vec![pair("catalog.rs", "src/catalog.rs"), pair("format.rs", "src/format.rs"), pair("compute.rs", "src/compute.rs"), pair("app.css", "style/app.css"), pair("views/leptos/views.rs", "src/views.rs")];
    }
    if self.kind == Kind::Dioxus {
      return vec![pair("catalog.rs", "src/catalog.rs"), pair("format.rs", "src/format.rs"), pair("compute.rs", "src/compute.rs"), pair("app.css", "assets/app.css"), pair("views/dioxus/views.rs", "src/views.rs")];
    }
    let (lib, css, views, ext, shared_views) = match self.kind {
      Kind::Fsr => ("app/src", "app/styles/app.css", "app/src/views", "tsx", "views"),
      Kind::Next => ("src", "app/app.css", "src/views", "tsx", "views"),
      Kind::SvelteKit => ("src/lib", "src/app.css", "src/lib/views", "svelte", "views/svelte"),
      _ => ("app/src", "app/assets/app.css", "app/src/views", "vue", "views/vue"),
    };
    let mut copies: Vec<(String, String)> = ["catalog.ts", "format.ts", "compute.ts"].iter().map(|f| pair(f, &format!("{lib}/{f}"))).collect();
    copies.push(pair("app.css", css));
    if self.kind == Kind::Fsr {
      copies.extend([pair("catalog.rs", "src/catalog.rs"), pair("compute.rs", "src/compute.rs")]);
    }
    copies.extend(["Primes", "Report", "Words", "Table"].iter().map(|v| pair(&format!("{shared_views}/{v}.{ext}"), &format!("{views}/{v}.{ext}"))));
    copies
  }
}

fn locked(lock: &str, krate: &str) -> Option<String> {
  let mut lines = lock.lines();
  while let Some(line) = lines.next() {
    if line == format!("name = \"{krate}\"") {
      return lines.next()?.strip_prefix("version = ").map(|v| v.trim_matches('"').to_owned());
    }
  }
  None
}

pub fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
  std::fs::create_dir_all(to)?;
  for entry in std::fs::read_dir(from)? {
    let entry = entry?;
    let kind = entry.file_type()?;
    let target = to.join(entry.file_name());
    if kind.is_dir() {
      copy_tree(&entry.path(), &target)?;
    } else if kind.is_symlink() {
      std::os::unix::fs::symlink(std::fs::read_link(entry.path())?, &target)?;
    } else {
      std::fs::copy(entry.path(), &target)?;
    }
  }
  Ok(())
}

/// Bytes and files under `dir`, symlinks counted as links.
pub fn tree_size(dir: &Path) -> (u64, u64) {
  let mut bytes = 0;
  let mut files = 0;
  let Ok(entries) = std::fs::read_dir(dir) else { return (0, 0) };
  for entry in entries.flatten() {
    let Ok(meta) = entry.path().symlink_metadata() else { continue };
    if meta.is_dir() {
      let (b, f) = tree_size(&entry.path());
      bytes += b;
      files += f;
    } else {
      bytes += meta.len();
      files += 1;
    }
  }
  (bytes, files)
}

/// What the Leptos server reads its configuration from in place of `Cargo.toml`, serving the site under `site` on `port`.
fn leptos_env(site: &Path, port: u16) -> Vec<(&'static str, String)> {
  vec![
    ("LEPTOS_OUTPUT_NAME", LEPTOS_BIN.to_owned()),
    ("LEPTOS_SITE_ROOT", site.display().to_string()),
    ("LEPTOS_SITE_PKG_DIR", "pkg".to_owned()),
    ("LEPTOS_SITE_ADDR", format!("127.0.0.1:{port}")),
    ("LEPTOS_ENV", "PROD".to_owned()),
  ]
}

/// The `dx` that bundles the Dioxus app: `$DX`, else the one `scripts/clean.sh` removes from `tools/`.
fn dx() -> PathBuf {
  std::env::var_os("DX").map_or_else(|| root().join("tools/bin/dx"), PathBuf::from)
}

/// Keeps the tools `dx` downloads under `tools/` instead of `~/.dx` and turns its usage reporting off.
fn dx_env() -> Vec<(&'static str, PathBuf)> {
  vec![("DX_HOME", root().join("tools/dx-home")), ("DX_TELEMETRY_ENABLED", PathBuf::from("false"))]
}

/// The `fsr` that generates and bundles the FSR app: `$FSR`, else the one on `PATH`.
fn fsr() -> String {
  std::env::var("FSR").unwrap_or_else(|_| "fsr".to_owned())
}
