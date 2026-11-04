# web_ssr_arena

The same online store built in SnapFire FSR, Next.js, SvelteKit, Nuxt, Leptos and Dioxus, then measured on one machine: how long it takes to build, how big it is to deploy, how much a browser downloads per page and how many requests a second it serves.

## Results

Requests per second at 128 concurrent clients, measured on an Apple M4 Pro (14 cores) in high power mode. Higher is better. Each figure is the median of its runs.

| Page | FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Home page | **171,576** | 8,430 | 32,079 | 29,283 | **164,653** | 36,091 |
| Product list | **53,208** | 851 | 6,167 | 1,929 | 35,044 | 13,049 |
| Product page | **121,024** | 1,481 | 13,171 | 3,722 | 102,790 | 32,175 |
| Search | **102,656** | 1,412 | 11,146 | 3,239 | 79,564 | 26,927 |
| JSON API | **169,760** | 6,717 | 24,814 | 39,999 | **169,373** | **170,573** |
| 500-row table | 11,182 | 186 | 1,157 | 1,233 | **14,137** | 10,686 |
| Prime sieve (heavy compute) | 870<br>**native 72,513** | 1,047 | 2,754 | 1,815 | **73,737** | 31,923 |
| Sales report (heavy compute) | 4,818<br>**native 98,020** | 1,487 | 9,787 | 3,589 | 92,004 | 32,953 |
| Word count (heavy compute) | 1,258<br>native 5,077 | 452 | 591 | 555 | **5,886** | 5,172 |

Bold marks the best figure in a row and every figure within 5% of it.

On the compute pages FSR is measured twice. The first figure has the loader computing in TypeScript. The one marked native has the loader calling the same Rust that Leptos and Dioxus run, through `#[native]`. The view and the render are FSR's both times. Both figures are ranked.

| Measure | FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Cold build | 40.5 s | 2.8 s | **1.5 s** | 3.4 s | 48.5 s | 56.9 s |
| Rebuild after one change | **0.7 s** | 1.4 s | 1.3 s | 2.6 s | 14.0 s | 13.2 s |
| Total to deploy, runtime included | 25.6 MiB | 151.8 MiB | 108.8 MiB | 109.7 MiB | 13.3 MiB | **8.6 MiB** |
| JavaScript per page, gzipped | 40.0 KiB | 135.0 KiB | 33.7 KiB | 69.3 KiB | **6.7 KiB** | 15.8 KiB |
| WebAssembly per page, gzipped | - | - | - | - | **257.1 KiB** | 483.2 KiB |
| Memory at start | 20.0 MB | 95.0 MB | 60.4 MB | 80.1 MB | **5.7 MB** | 10.2 MB |
| Start to first response | **16 ms** | 322 ms | 54 ms | 212 ms | 30 ms | 24 ms |

The total to deploy is the deploy tree plus the runtime installed beside it: Node, 107.3 MiB, for Next.js, SvelteKit and Nuxt. Bytes per page are for the counter page.

Every number, every concurrency level and the method are in [docs/README.md](docs/README.md).

## What's in here

| Path | What it is |
| :--- | :--- |
| `apps/fsr/` | The store in FSR |
| `apps/nextjs/` | The store in Next.js |
| `apps/sveltekit/` | The store in SvelteKit |
| `apps/nuxt/` | The store in Nuxt |
| `apps/leptos/` | The store in Leptos |
| `apps/dioxus/` | The store in Dioxus |
| `shared/` | The data, styles and page code every app uses |
| `src/` | The benchmark harness |
| `load/` | The HTTP load client the harness drives, with `loadgen` and `matrix` for loading one server by hand |
| `docs/` | The results |
| `scripts/clean.sh` | Removes everything the arena writes |

## Setup

```sh
(cd apps/nextjs && npm install)
(cd apps/sveltekit && npm install)
(cd apps/nuxt && npm install)
cargo build --release
```

Leptos builds with `cargo leptos`, whose wasm-bindgen has to match the one in `apps/leptos/Cargo.lock`. The lockfile holds wasm-bindgen 0.2.127 for cargo-leptos 0.3.7.

Dioxus builds with `dx`, kept inside the arena rather than installed:

```sh
cargo install dioxus-cli --version 0.7.10 --locked --root tools
```

The harness runs `$DX`, else `tools/bin/dx`, with `DX_HOME=tools/dx-home` so the wasm-bindgen, esbuild and binaryen it downloads stay under `tools/`. It also sets `DX_TELEMETRY_ENABLED=false`.

## Cleaning up

```sh
scripts/clean.sh                  # everything the arena writes, installs included
scripts/clean.sh --keep-installs  # keep node_modules/, FSR's fetched types and tools/
```

Rust targets go through `cargo clean`, the cold build directories under `runs/` included. Node build output, generated FSR files and `runs/` are removed. Nothing outside the arena is touched: Cargo's registry, npm's cache and the rustc wrapper's store are shared with other projects.

`build` generates the FSR app with `fsr build app` before cargo compiles it. `size` packs its deploy tree with `fsr bundle`. Both run `$FSR`, else `fsr` from `PATH`, which has to match the FSR crates in `apps/fsr/Cargo.toml`. Build one from the checkout the patch block points at:

```sh
cargo build --release -p snapfire_fsr_cli --bin fsr --manifest-path ../snapfire/fsr/Cargo.toml
export FSR=$PWD/../snapfire/fsr/target/release/fsr
```

## Commands

```sh
./target/release/web_ssr_arena list     # the apps, pages and cells, nothing run
./target/release/web_ssr_arena build    # a cold build, then three warm builds, of each app; leaves each built
./target/release/web_ssr_arena check    # every page of every app answers what it must
./target/release/web_ssr_arena size     # deploy tree, binary, runtime and the bytes each page loads
./target/release/web_ssr_arena run      # every page at 1, 8, 32, 64, 128 and 256 clients, 10 s cells, three repetitions
./target/release/web_ssr_arena report   # rewrite docs/ from docs/raw/, measure nothing
```

The default run is 6 apps × 11 pages × 6 levels × 3 repetitions at 10 s, plus FSR's three native compute pages, plus 3 s of warm-up per page: 1,242 cells in about 218 minutes. `build` and `run` refuse to start outside high power mode unless given `--any-power`. Both record the power mode and load at both ends.

### Running part of it

```sh
./target/release/web_ssr_arena run --app fsr --page catalog,primes --levels 1,64 --secs 10 --reps 1
./target/release/web_ssr_arena build --no-cold --reps 3
```

`run` takes `--app`, `--page`, `--levels`, `--secs`, `--warmup` and `--reps`. `build` takes `--reps` and `--no-cold`. A partial `run` writes `docs/raw/perf.json` over the full one, so copy it aside first.

### Resuming a stopped run

`run` writes each cell to `runs/perf.jsonl` as it lands. A stopped run continues from there, measuring only the cells the journal lacks. `docs/raw/perf.json` then holds every cell with each sitting's machine state:

```sh
./target/release/web_ssr_arena run --resume
```

`run` refuses to start over an existing journal without `--resume`. A run printed before the journal existed is brought in from its output with `import-log`; its cells carry requests per second, p50, p99, CPU and memory, the fields the output prints:

```sh
./target/release/web_ssr_arena import-log runs/run-2026-11-03.log
```

### Before a run

Nothing else should be running. The load client shares the machine with the server and takes four threads.

## The Pages

| Page | Path | Rendered | What it holds |
| :--- | :--- | :--- | :--- |
| `home` | `/` | at build | A hero and three features, no data. |
| `counter` | `/counter` | at build | One component that hydrates in the browser. |
| `catalog` | `/products?sort=price` | per request | All 48 products, sorted by the query. |
| `product` | `/products/cedar-lantern` | per request | One product by its slug, with four related products. |
| `search` | `/search?q=pack` | per request | The 12 products matching the query. |
| `missing` | `/products/nope` | per request | A 404. |
| `api` | `/api/products` | per request | The catalog as JSON. |
| `primes` | `/compute/primes?n=50000` | per request | A sieve of Eratosthenes up to `n`: tight numeric loops over an array. |
| `report` | `/compute/report?orders=5000` | per request | Generated orders aggregated per product, sorted by revenue with a comparator and formatted. |
| `words` | `/compute/words?passes=20` | per request | Word frequencies over a built-up text: splitting, lowercasing and counting in an object. |
| `table` | `/table?rows=500` | per request | 500 rows from simple data, so the cost is the render. |

Every page is fetched and checked for its status, its content type, its text and its count of product cards before any load runs against it. The compute pages and the table each write a digest of what they computed. A run refuses to start when two apps write different digests:

```sh
./target/release/web_ssr_arena check
# fsr        every page answers as it must
#            primes   primes 50000 5133 121013308 49919,49921,...
```

Their code is `shared/compute.ts` and the views under `shared/views/`, copied into every app like the catalog. FSR and Next.js share the TSX views; SvelteKit takes the Svelte ones in `shared/views/svelte/`, Nuxt the Vue ones in `shared/views/vue/`, Leptos the components in `shared/views/leptos/` and Dioxus those in `shared/views/dioxus/`. Leptos and Dioxus take the catalog and compute code as `shared/catalog.rs`, `shared/compute.rs` and `shared/format.rs`, the same code in Rust. `check` fails when a copy differs from `shared/`.

FSR is measured on `primes`, `report` and `words` a second time, under `/compute/native/`, with the loader calling `shared/compute.rs` through `#[native]` (`apps/fsr/src/native.rs`). The view and the render stay FSR's. These cells are recorded as `primes.native`, `report.native` and `words.native`. `report` writes each figure under FSR's own in the same table cell. `check` requires each native page to write the same digest as its TypeScript form.

## Measures

| Command | Measure | How |
| :--- | :--- | :--- |
| `build` | Cold build | From empty output directories, dependencies downloaded: a fresh `CARGO_TARGET_DIR` and no rustc wrapper for FSR, Leptos and Dioxus, a fresh `distDir` for Next.js, fresh output, build and Vite cache directories for SvelteKit and Nuxt. FSR's time includes `fsr build app` and its `prerender` step. |
| `build` | Warm build | One page touched, everything else reused. |
| `size` | Deploy tree | For FSR, `fsr bundle` plus the binary and `config/`. For Next.js, `.next/standalone` with `.next/static` copied in. For SvelteKit, `build/` and `package.json`. For Nuxt, `.output/`. For Leptos, its `site/` and the binary. For Dioxus, the `web/` directory `dx` writes. Node is listed apart as the runtime the last three need. |
| `size` | Bytes per page | The document, stylesheets, preloaded WebAssembly and scripts, following module imports, raw and gzipped. Modules reached only by a dynamic `import()` are counted apart. |
| `run` | Requests per second and latency | One HTTP/1.1 connection per client, the next request sent when the last body is read, from the client in `load/`. |
| `run` | CPU and memory | The server's CPU over each cell from `ps`, its resident set after each cell and its peak from `wait4` when it stops. Spawn to first 200 at each start. |

FSR, Leptos and Dioxus serve on every core. Next.js, SvelteKit and Nuxt each serve from one Node process.

## Results files

| Path | What it is |
| :--- | :--- |
| `docs/README.md` | Every table, the machine state, the method and the caveats |
| `docs/raw/build.json`, `size.json`, `perf.json` | What each measuring command wrote |
| `docs/raw/results.json` | The results in the arena format the excsn site reads |
| `runs/` | Logs, deploy trees and cold build directories, ignored by git |

```sh
./target/release/web_ssr_arena report
# wrote docs/README.md
# wrote docs/raw/results.json
```
