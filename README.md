# web_ssr_arena

The same online store built in SnapFire FSR, Next.js, SvelteKit, Nuxt, Leptos and Dioxus, then measured on one machine: how long it takes to build, how big it is to deploy, how much a browser downloads per page and how many requests a second it serves.

## Results

Requests per second at 128 concurrent clients, measured on an Apple M4 Pro (14 cores) in high power mode. Higher is better. Each figure is the median of its runs.

| Page | FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Home page | **171,551** | 8,430 | 32,079 | 29,283 | **164,653** | 36,091 |
| Product list | **58,858** | 851 | 6,167 | 1,929 | 35,044 | 13,049 |
| Product page | **124,144** | 1,481 | 13,171 | 3,722 | 102,790 | 32,175 |
| Search | **106,410** | 1,412 | 11,146 | 3,239 | 79,564 | 26,927 |
| JSON API | **167,587** | 6,717 | 24,814 | 39,999 | **169,373** | **170,573** |
| 500-row table | **15,698** | 186 | 1,157 | 1,233 | 14,137 | 10,686 |
| Prime sieve (heavy compute) | 1,872<br>**native 83,666** | 1,047 | 2,754 | 1,815 | 73,737 | 31,923 |
| Sales report (heavy compute) | 6,541<br>**native 110,928** | 1,487 | 9,787 | 3,589 | 92,004 | 32,953 |
| Word count (heavy compute) | 1,450<br>**native 5,729** | 452 | 591 | 555 | **5,886** | 5,172 |

Bold marks the best figure in a row and every figure within 5% of it.

On the compute pages FSR is measured twice. The first figure has the loader computing in TypeScript. The one marked native has the loader calling the same Rust that Leptos and Dioxus run, through `#[native]`. The view and the render are FSR's both times. Both figures are ranked.

| Measure | FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Cold build | 46.7 s | 2.8 s | **1.5 s** | 3.4 s | 48.5 s | 56.9 s |
| Rebuild after one change | **0.7 s** | 1.4 s | 1.3 s | 2.6 s | 14.0 s | 13.2 s |
| Total to deploy, runtime included | 28.2 MiB | 151.8 MiB | 108.8 MiB | 109.7 MiB | 13.3 MiB | **8.6 MiB** |
| JavaScript per page, gzipped | 40.0 KiB | 135.0 KiB | 33.7 KiB | 69.3 KiB | **6.7 KiB** | 15.8 KiB |
| WebAssembly per page, gzipped | - | - | - | - | **257.1 KiB** | 483.2 KiB |
| Memory at start | 19.5 MB | 95.0 MB | 60.4 MB | 80.1 MB | **5.7 MB** | 10.2 MB |
| Start to first response | **16 ms** | 322 ms | 54 ms | 212 ms | 30 ms | 24 ms |

The total to deploy is the deploy tree plus the runtime installed beside it: Node, 107.3 MiB, for Next.js, SvelteKit and Nuxt. Bytes per page are for the counter page.

Every number, every concurrency level and the method are in [docs/README.md](docs/README.md).

## Benchmarks

Eleven pages are loaded at 1 to 256 concurrent clients in every app: two built once, four rendered per request from the catalog, a JSON API, three compute-heavy pages and a 500-row table. Build time, deploy size, startup, memory and the bytes each page loads are measured beside them. Every page is checked to answer the same in every app before any load runs. 

[BENCHMARK_SPECS.md](BENCHMARK_SPECS.md) has each benchmark, the pages, the machine it needs, the load and how each measure is taken.

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
| `BENCHMARK_SPECS.md` | What is measured and how |
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

`build` generates the FSR app with `fsr build app` before cargo compiles it. `size` packs its deploy tree with `fsr bundle`. Both run `$FSR`, else `fsr` from `PATH`, which has to match the FSR crates in `apps/fsr/Cargo.toml`. Build one from the snapfire checkout the FSR crates come from:

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
