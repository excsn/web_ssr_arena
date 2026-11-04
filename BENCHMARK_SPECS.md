# Benchmark specs

What the arena measures, on what and under which conditions. The commands that run it are in [README.md](README.md) and the results are in [docs/README.md](docs/README.md).

## Benchmarks

| Benchmark | Command | What it runs | Reported as | Better |
| :--- | :--- | :--- | :--- | :--- |
| Home page | `run` | `home`: a page rendered at build, no data | Requests per second, p50 and p99 latency, CPU and memory at each concurrency level | Higher requests, lower latency |
| Hydrating page | `run` | `counter`: a page rendered at build holding one hydrating component | As above | As above |
| Product list | `run` | `catalog`: 48 products sorted per request | As above | As above |
| Product page | `run` | `product`: one product by slug with four related | As above | As above |
| Search | `run` | `search`: products matching a query | As above | As above |
| Not found | `run` | `missing`: a 404 rendered per request | As above | As above |
| JSON API | `run` | `api`: the catalog serialized as JSON | As above | As above |
| 500-row table | `run` | `table`: 500 rows rendered from simple data | As above | As above |
| Prime sieve | `run` | `primes` plus `primes.native` for FSR: numeric loops over an array | As above | As above |
| Sales report | `run` | `report` plus `report.native` for FSR: aggregation, a comparator sort and formatting | As above | As above |
| Word count | `run` | `words` plus `words.native` for FSR: string splitting and counting in an object | As above | As above |
| Startup | `run` | Every server start | Spawn to first 200 and resident set at that point | Lower |
| Peak memory | `run` | Every server over its whole run | Peak resident set | Lower |
| Cold build | `build` | A build from empty output directories | Seconds | Lower |
| Warm build | `build` | A rebuild after one page is touched | Seconds | Lower |
| Deploy size | `size` | The deploy tree, the server binary and the runtime installed beside it | Bytes | Lower |
| Bytes per page | `size` | What a browser loads for each page: document, stylesheets, scripts and WebAssembly | Raw and gzipped bytes | Lower |

## Machine

| Requirement | Spec |
| :--- | :--- |
| Hardware | Apple M4 Pro, 14 cores |
| Power mode | High power (`pmset` powermode 2). `build` and `run` refuse to start in any other mode unless given `--any-power`. |
| Machine state | Power mode and load average recorded at the start and end of every sitting, beside its results |
| Other work | Nothing else running. The load client shares the machine with the server. |

## Applications

| App | Framework | Serves from |
| :--- | :--- | :--- |
| `fsr` | SnapFire FSR | One process on every core |
| `nextjs` | Next.js | One Node process |
| `sveltekit` | SvelteKit | One Node process |
| `nuxt` | Nuxt | One Node process |
| `leptos` | Leptos | One process on every core |
| `dioxus` | Dioxus | One process on every core |

Every app is the same storefront: 48 products, the same stylesheet and the same markup. The data and compute code are written once in TypeScript (`shared/catalog.ts`, `shared/compute.ts`) and once in Rust (`shared/catalog.rs`, `shared/compute.rs`, `shared/format.rs`) for Leptos and Dioxus. FSR and Next.js share the TSX views. SvelteKit, Nuxt, Leptos and Dioxus each have the same views written in their own component language under `shared/views/`. `check` fails when an app's copy differs from `shared/`.

Each result records the version of the app that produced it: the framework's package version and, for FSR, the snapfire commit with a hash of any uncommitted changes.

## Pages

| Page | Path | Rendered | Workload |
| :--- | :--- | :--- | :--- |
| `home` | `/` | At build | A hero and three features, no data |
| `counter` | `/counter` | At build | One component that hydrates in the browser |
| `catalog` | `/products?sort=price` | Per request | All 48 products, sorted by the query |
| `product` | `/products/cedar-lantern` | Per request | One product by its slug, with four related products |
| `search` | `/search?q=pack` | Per request | The 12 products whose name or tag matches the query |
| `missing` | `/products/nope` | Per request | A slug with no product, answered 404 |
| `api` | `/api/products` | Per request | The catalog as JSON |
| `primes` | `/compute/primes?n=50000` | Per request | A sieve of Eratosthenes up to `n`: tight numeric loops over an array |
| `report` | `/compute/report?orders=5000` | Per request | Generated orders aggregated per product, sorted by revenue with a comparator and formatted |
| `words` | `/compute/words?passes=20` | Per request | Word frequencies over a built-up text: splitting, lowercasing and counting in an object |
| `table` | `/table?rows=500` | Per request | 500 rows from simple data, so the cost is the render |

### FSR native forms

FSR runs `primes`, `report` and `words` a second time under `/compute/native/`, with the loader calling `shared/compute.rs` through `#[native]` (`apps/fsr/src/native.rs`). The view and the render stay FSR's. These cells are `primes.native`, `report.native` and `words.native`, shown under FSR's own figure in the same table cell and ranked with the row.

## Correctness

Before any load runs against a page, every app's page is fetched and checked for its status, its content type, its text and its count of product cards. The compute pages and the table write a digest of what they computed. A run refuses to start when two apps write different digests. Each FSR native page has to write the digest of its TypeScript form.

```sh
./target/release/web_ssr_arena check
# fsr        every page answers as it must
#            primes   primes 50000 5133 121013308 49919,49921,...
```

## Load

| Parameter | Spec | Flag |
| :--- | :--- | :--- |
| Client | `load/`, HTTP/1.1, one connection per concurrent client, the next request sent when the last body is read | |
| Client threads | 4 | |
| Concurrency levels | 1, 8, 32, 64, 128 and 256 clients | `--levels` |
| Cell length | 10 s | `--secs` |
| Warm-up | 3 s of load on each page before its cells | `--warmup` |
| Repetitions | Interleaved across apps, their order flipping between repetitions. Each repetition starts every server afresh. | `--reps` |
| Journal | Every cell is written to `runs/perf.jsonl` as it lands, so a stopped run resumes with `--resume` | |

## Measures

| Measure | Command | How |
| :--- | :--- | :--- |
| Requests per second | `run` | Responses read in full per second over the cell |
| Latency | `run` | From sending a request to having read its body, p50 and p99 |
| CPU | `run` | The server's CPU over each cell, from `ps` |
| Memory | `run` | The server's resident set after each cell, its peak from `wait4` when it stops |
| Startup | `run` | Spawn to first 200 at each server start with the resident set at that point |
| Cold build | `build` | From empty output directories with dependencies downloaded: a fresh `CARGO_TARGET_DIR` and no rustc wrapper for FSR, Leptos and Dioxus, a fresh `distDir` for Next.js, fresh output, build and Vite cache directories for SvelteKit and Nuxt. FSR's time includes `fsr build app` and its `prerender` step. |
| Warm build | `build` | One page touched, everything else reused |
| Deploy tree | `size` | For FSR, `fsr bundle` plus the binary and `config/`. For Next.js, `.next/standalone` with `.next/static` copied in. For SvelteKit, `build/` and `package.json`. For Nuxt, `.output/`. For Leptos, its `site/` and the binary. For Dioxus, the `web/` directory `dx` writes. |
| Runtime | `size` | Node for Next.js, SvelteKit and Nuxt, counted into the total to deploy |
| Bytes per page | `size` | The document, stylesheets, preloaded WebAssembly and scripts, following module imports, raw and gzipped. Modules reached only by a dynamic `import()` are counted apart. |

`build` and `size` run for one app replace only that app's results and keep the others.

## Reporting

| Rule | Spec |
| :--- | :--- |
| Direction | Higher is better for requests per second. Lower is better for latency, memory, build time and size. |
| Best | Bold marks the best figure in a row and every figure within 5% of it. A row whose figures are all the same has no best. |
| Missing | An app without a figure shows `-` and is not ranked |
| Output | `report` writes `docs/README.md` and `docs/raw/results.json` from `docs/raw/`, measuring nothing |
