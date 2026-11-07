# Arena results

The same storefront written in SnapFire FSR, Next.js, SvelteKit, Nuxt, Leptos and Dioxus, built for production and served on one machine: build time, deploy size, bytes per page and requests per second.

Higher is better for requests per second. Lower is better for latency, memory, build time and size. Bold marks the best figure in a row and every figure within 5% of it.

Machine: Apple M4 Pro (14 cores).

- Dioxus: dioxus 0.7.10
- SnapFire FSR: snapfire_fsr_host 0.23.0 at 907014e6+dirty.5fefbbfe for home, counter, catalog, product, search, missing, api, primes, primes.native, report, report.native, words, words.native and table; snapfire_fsr_host 0.23.0 at 907014e6+dirty.759746d9 for the build and the sizes
- Leptos: leptos 0.8.23
- Next.js: next 16.4.4, react 19.3.1
- Nuxt: nuxt 4.6.4, vue 3.5.44
- SvelteKit: @sveltejs/kit 3.1.1, svelte 5.57.3

Load: 10 s cells after 3 s of warm-up per page, 4 client threads. Started 2026-11-03T16:53:25Z in high power power mode at load 6.23. ended 2026-11-07T23:01:10Z in high power power mode at load 19.93.

Build: Started 2026-11-07T20:16:35Z in high power power mode at load 14.28. ended 2026-11-07T20:17:24Z in high power power mode at load 23.3 for SnapFire FSR; Started 2026-11-03T16:47:59Z in high power power mode at load 20.43. ended 2026-11-03T16:52:52Z in high power power mode at load 9.25 for Next.js, SvelteKit, Nuxt, Leptos and Dioxus.

Size: measured 2026-11-07T20:17:35Z for SnapFire FSR; 2026-11-03T16:53:07Z for Next.js, SvelteKit, Nuxt, Leptos and Dioxus.

## Requests per second

Responses read in full per second at each number of concurrent clients.

#### home: `/`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **42,844** | 7,632 | 13,496 | 10,983 | 22,020 | 10,389 |
| 8 | **150,027** | 8,741 | 26,369 | 29,170 | 99,845 | 34,585 |
| 32 | **175,288** | 8,541 | 28,770 | 30,133 | 155,975 | 36,077 |
| 64 | **171,856** | 8,557 | 32,114 | 30,201 | 162,622 | 35,995 |
| 128 | **169,908** | 8,430 | 32,079 | 29,283 | **164,653** | 36,091 |
| 256 | **169,009** | 8,404 | 29,435 | 28,854 | **169,079** | 36,246 |

The landing page: a hero and three features, no data.

#### counter: `/counter`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **42,677** | 7,602 | 14,694 | 10,865 | 23,910 | 11,068 |
| 8 | **150,083** | 8,623 | 30,502 | 28,784 | 102,435 | 34,746 |
| 32 | **175,050** | 8,644 | 31,196 | 29,812 | 157,534 | 36,088 |
| 64 | **170,622** | 8,632 | 31,756 | 29,670 | **163,478** | 36,238 |
| 128 | **168,899** | 8,547 | 32,016 | 29,000 | **167,743** | 36,344 |
| 256 | **167,729** | 8,451 | 29,176 | 28,392 | **168,051** | 36,604 |

A fixed page holding one component that hydrates in the browser.

#### catalog: `/products?sort=price`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **6,979** | 826 | 5,872 | 1,765 | 4,138 | 3,330 |
| 8 | **38,195** | 904 | 6,441 | 2,055 | 23,157 | 14,066 |
| 32 | **55,021** | 881 | 6,487 | 1,999 | 35,330 | 13,006 |
| 64 | **54,780** | 865 | 6,436 | 1,868 | 35,408 | 13,013 |
| 128 | **55,027** | 851 | 6,167 | 1,929 | 35,044 | 13,049 |
| 256 | **53,889** | 830 | 6,156 | 1,916 | 33,814 | 13,002 |

All 48 products, sorted by the query.

#### product: `/products/cedar-lantern`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **15,169** | 1,323 | 11,031 | 3,096 | 12,099 | 8,664 |
| 8 | **71,253** | 1,558 | 13,790 | 3,829 | 55,815 | 30,738 |
| 32 | **116,484** | 1,514 | 13,800 | 3,893 | 93,882 | 32,289 |
| 64 | **116,986** | 1,459 | 13,624 | 3,793 | 99,872 | 32,142 |
| 128 | **118,627** | 1,481 | 13,171 | 3,722 | 102,790 | 32,175 |
| 256 | **120,903** | 1,518 | 12,656 | 3,696 | 102,065 | 32,049 |

One product by its slug, with four related products.

#### search: `/search?q=pack`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **12,243** | 1,291 | 9,678 | 2,732 | 9,099 | 6,815 |
| 8 | **61,339** | 1,482 | 11,317 | 3,379 | 45,691 | 25,539 |
| 32 | **102,263** | 1,472 | 11,520 | 3,344 | 78,166 | 26,985 |
| 64 | **102,088** | 1,402 | 11,440 | 3,246 | 79,932 | 26,811 |
| 128 | **102,374** | 1,412 | 11,146 | 3,239 | 79,564 | 26,927 |
| 256 | **103,391** | 1,414 | 10,714 | 3,169 | 80,449 | 26,017 |

Products whose name or tag matches the query.

#### missing: `/products/nope`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **18,745** | 1,149 | 12,632 | 1,924 | 16,644 | 10,177 |
| 8 | **84,120** | 1,297 | 16,127 | 2,398 | 67,289 | 32,835 |
| 32 | **135,717** | 1,280 | 16,113 | 2,420 | 105,319 | 34,701 |
| 64 | **140,978** | 1,236 | 15,889 | 2,367 | 110,110 | 34,345 |
| 128 | **143,811** | 1,243 | 15,576 | 2,319 | 113,409 | 34,953 |
| 256 | **148,869** | 1,254 | 14,961 | 2,329 | 115,669 | 35,108 |

A slug with no product, answered 404.

#### api: `/api/products`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 22,305 | 6,410 | 17,402 | 22,715 | **30,810** | 18,327 |
| 8 | 107,480 | 6,788 | 25,451 | 40,578 | **127,998** | 79,967 |
| 32 | 162,645 | 6,712 | 25,338 | 42,368 | **172,697** | 149,825 |
| 64 | **164,910** | 6,710 | 25,349 | 41,314 | **171,057** | **163,376** |
| 128 | **167,442** | 6,717 | 24,814 | 39,999 | **169,373** | **170,573** |
| 256 | **170,712** | 6,710 | 23,755 | 39,614 | **168,457** | **171,040** |

The catalog as JSON.

#### primes: `/compute/primes?n=50000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 204<br>**native 8,678** | 963 | 2,435 | 1,498 | 8,218 | 6,203 |
| 8 | 1,333<br>**native 48,344** | 1,061 | 2,768 | 1,835 | 42,043 | 29,294 |
| 32 | 1,848<br>**native 79,217** | 1,062 | 2,564 | 1,742 | 73,409 | 31,566 |
| 64 | 1,831<br>**native 80,079** | 1,049 | 2,531 | 1,720 | 73,763 | 32,048 |
| 128 | 1,839<br>**native 80,995** | 1,047 | 2,754 | 1,815 | 73,737 | 31,923 |
| 256 | 1,825<br>**native 81,177** | 1,054 | 2,766 | 1,815 | 73,489 | 31,952 |

A sieve of Eratosthenes up to the query's n: tight numeric loops over an array.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: `/compute/report?orders=5000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 618<br>**native 12,230** | 1,372 | 8,665 | 3,053 | 10,499 | 7,751 |
| 8 | 4,378<br>**native 61,590** | 1,551 | 10,183 | 3,841 | 51,338 | 30,923 |
| 32 | 6,381<br>**native 104,390** | 1,562 | 10,147 | 3,789 | 88,265 | 32,928 |
| 64 | 6,280<br>**native 106,232** | 1,523 | 10,004 | 3,645 | 90,790 | 33,102 |
| 128 | 6,252<br>**native 106,933** | 1,487 | 9,787 | 3,589 | 92,004 | 32,953 |
| 256 | 6,242<br>**native 106,979** | 1,481 | 9,522 | 3,645 | 92,903 | 32,923 |

Generated orders aggregated per product, sorted by revenue with a comparator and formatted.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: `/compute/words?passes=20`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 139<br>native 522 | 433 | **577** | 542 | 542 | 520 |
| 8 | 945<br>native 3,745 | 454 | 601 | 555 | **4,037** | **3,890** |
| 32 | 1,383<br>native 5,400 | 450 | 594 | 551 | **5,900** | 5,180 |
| 64 | 1,370<br>native 5,345 | 449 | 601 | 545 | **5,913** | 5,057 |
| 128 | 1,365<br>native 5,392 | 452 | 591 | 555 | **5,886** | 5,172 |
| 256 | 1,357<br>native 5,320 | 450 | 518 | 489 | **5,818** | 5,081 |

Word frequencies over a built-up text: splitting, lowercasing and counting in an object.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: `/table?rows=500`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **1,517** | 167 | 1,092 | 1,172 | **1,518** | 1,368 |
| 8 | **10,846** | 172 | 1,159 | 1,287 | **10,311** | 8,872 |
| 32 | **14,936** | 181 | 1,132 | 1,243 | **14,217** | 10,834 |
| 64 | **14,887** | 184 | 1,150 | 1,223 | 14,013 | 10,763 |
| 128 | **14,817** | 186 | 1,157 | 1,233 | **14,137** | 10,686 |
| 256 | **14,706** | 169 | 1,160 | 1,248 | **14,127** | 10,803 |

500 rows rendered from simple data: the cost is the render, not the loader.

## Latency

From sending a request to having read its body, median and 99th percentile.

#### home: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.02 / 0.04** | 0.12 / 0.39 | 0.07 / 0.26 | 0.09 / 0.11 | 0.04 / 0.06 | 0.10 / 0.13 |
| 8 | **0.05 / 0.09** | 0.77 / 1.93 | 0.26 / 1.15 | 0.26 / 0.55 | 0.08 / 0.12 | 0.21 / 0.47 |
| 32 | **0.18 / 0.28** | 3.56 / 7.31 | 1.00 / 2.19 | 1.00 / 1.52 | 0.20 / 0.35 | 0.79 / 2.45 |
| 64 | **0.37 / 0.54** | 7.35 / 10.84 | 1.97 / 2.57 | 2.02 / 2.91 | 0.38 / 0.71 | 1.68 / 3.77 |
| 128 | **0.75 / 1.03** | 14.76 / 22.07 | 4.00 / 4.74 | 4.31 / 5.74 | 0.75 / 1.43 | 3.45 / 6.19 |
| 256 | **1.50 / 1.93** | 29.65 / 44.12 | 8.52 / 11.97 | 8.75 / 11.00 | 1.45 / 2.94 | 6.97 / 10.56 |

Ranked on p99.

#### counter: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.02 / 0.04** | 0.12 / 0.40 | 0.07 / 0.10 | 0.09 / 0.11 | 0.04 / 0.06 | 0.09 / 0.12 |
| 8 | **0.05 / 0.09** | 0.78 / 1.96 | 0.25 / 0.47 | 0.27 / 0.55 | 0.08 / 0.12 | 0.21 / 0.48 |
| 32 | **0.18 / 0.28** | 3.49 / 7.22 | 0.99 / 1.44 | 1.02 / 1.55 | 0.20 / 0.35 | 0.79 / 2.46 |
| 64 | **0.37 / 0.54** | 7.32 / 11.10 | 1.98 / 2.62 | 2.04 / 3.00 | 0.38 / 0.70 | 1.67 / 3.81 |
| 128 | **0.75 / 1.03** | 14.78 / 20.95 | 4.00 / 4.83 | 4.32 / 5.87 | 0.74 / 1.42 | 3.43 / 6.16 |
| 256 | **1.52 / 1.95** | 29.69 / 43.53 | 8.53 / 11.91 | 8.91 / 11.45 | 1.47 / 2.93 | 6.89 / 10.64 |

Ranked on p99.

#### catalog: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.14 / 0.16** | 1.12 / 1.93 | 0.16 / 0.38 | 0.54 / 0.92 | 0.24 / 0.27 | 0.30 / 0.34 |
| 8 | **0.20 / 0.28** | 8.67 / 11.93 | 1.03 / 2.48 | 3.96 / 6.57 | 0.34 / 0.43 | 0.54 / 1.04 |
| 32 | **0.56 / 1.25** | 35.69 / 44.74 | 4.86 / 7.96 | 13.48 / 35.23 | 0.87 / 1.83 | 2.41 / 4.47 |
| 64 | **1.15 / 2.29** | 71.75 / 92.39 | 9.72 / 16.28 | 30.10 / 78.66 | 1.76 / 3.61 | 4.85 / 7.38 |
| 128 | **2.32 / 4.70** | 141.70 / 308.39 | 19.70 / 33.19 | 60.15 / 127.95 | 3.49 / 8.13 | 9.71 / 13.38 |
| 256 | **4.64 / 10.68** | 215.52 / 4341.42 | 38.83 / 61.20 | 75.48 / 810.49 | 6.88 / 20.06 | 19.59 / 25.35 |

Ranked on p99.

#### product: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.06 / 0.08** | 0.69 / 1.47 | 0.09 / 0.36 | 0.29 / 0.96 | 0.08 / 0.10 | 0.11 / 0.14 |
| 8 | **0.11 / 0.16** | 5.09 / 7.68 | 0.47 / 1.33 | 1.84 / 4.29 | 0.14 / 0.21 | 0.24 / 0.48 |
| 32 | **0.26 / 0.56** | 20.57 / 27.78 | 2.15 / 3.95 | 8.95 / 16.77 | 0.33 / 0.61 | 0.90 / 2.51 |
| 64 | **0.53 / 1.11** | 42.86 / 54.46 | 4.66 / 8.00 | 15.12 / 37.05 | 0.62 / 1.18 | 1.91 / 3.93 |
| 128 | **1.05 / 2.24** | 83.64 / 106.71 | 9.33 / 16.52 | 27.02 / 77.04 | 1.17 / 2.77 | 3.89 / 6.49 |
| 256 | **2.02 / 4.77** | 156.84 / 996.46 | 19.27 / 33.15 | 50.65 / 130.54 | 2.29 / 6.44 | 7.89 / 11.54 |

Ranked on p99.

#### search: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.08 / 0.10** | 0.72 / 1.40 | 0.10 / 0.40 | 0.34 / 0.93 | 0.11 / 0.13 | 0.14 / 0.21 |
| 8 | **0.13 / 0.18** | 5.42 / 7.95 | 0.58 / 1.63 | 2.15 / 4.90 | 0.17 / 0.25 | 0.29 / 0.57 |
| 32 | **0.30 / 0.66** | 21.26 / 28.38 | 2.67 / 4.67 | 10.54 / 19.92 | 0.39 / 0.98 | 1.10 / 2.74 |
| 64 | **0.61 / 1.25** | 44.45 / 56.45 | 5.52 / 9.21 | 17.21 / 44.35 | 0.77 / 1.69 | 2.30 / 4.37 |
| 128 | **1.22 / 2.56** | 86.82 / 110.80 | 11.14 / 18.78 | 30.86 / 88.20 | 1.53 / 3.64 | 4.67 / 7.31 |
| 256 | **2.39 / 5.58** | 166.36 / 1203.57 | 22.63 / 38.23 | 56.50 / 156.54 | 2.90 / 8.22 | 9.54 / 13.93 |

Ranked on p99.

#### missing: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.05 / 0.07** | 0.81 / 1.55 | 0.07 / 0.38 | 0.46 / 1.28 | 0.06 / 0.08 | 0.09 / 0.16 |
| 8 | **0.09 / 0.14** | 6.09 / 9.24 | 0.40 / 1.21 | 3.02 / 6.37 | 0.12 / 0.18 | 0.22 / 0.50 |
| 32 | **0.23 / 0.43** | 24.25 / 33.48 | 1.84 / 3.48 | 12.85 / 19.89 | 0.30 / 0.47 | 0.82 / 2.51 |
| 64 | **0.44 / 0.87** | 50.29 / 64.34 | 3.97 / 7.20 | 25.84 / 38.47 | **0.58 / 0.88** | 1.76 / 3.95 |
| 128 | **0.86 / 1.80** | 98.60 / 128.41 | 7.97 / 13.75 | 51.74 / 79.19 | **1.12 / 1.72** | 3.57 / 6.27 |
| 256 | 1.64 / 3.86 | 173.43 / 1745.76 | 16.24 / 28.26 | 95.71 / 134.89 | **2.18 / 3.44** | 7.20 / 10.80 |

Ranked on p99.

#### api: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.04 / 0.06 | 0.14 / 0.53 | 0.05 / 0.11 | 0.04 / 0.06 | **0.03 / 0.05** | 0.05 / 0.07 |
| 8 | 0.07 / 0.12 | 1.08 / 2.35 | 0.25 / 0.77 | 0.16 / 0.48 | **0.06 / 0.10** | 0.10 / 0.14 |
| 32 | 0.19 / 0.35 | 4.70 / 8.66 | 1.16 / 2.81 | 0.69 / 1.56 | **0.18 / 0.29** | 0.21 / 0.37 |
| 64 | 0.37 / 0.73 | 9.26 / 15.31 | 2.35 / 4.56 | 1.44 / 3.08 | **0.37 / 0.55** | 0.38 / 0.72 |
| 128 | 0.72 / 1.52 | 18.32 / 29.94 | 5.02 / 8.61 | 3.07 / 5.06 | **0.75 / 0.99** | 0.73 / 1.36 |
| 256 | 1.41 / 3.10 | 36.50 / 59.72 | 10.37 / 17.35 | 6.34 / 7.99 | **1.51 / 1.82** | 1.44 / 2.70 |

Ranked on p99.

#### primes: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 4.90 / 5.14<br>**native 0.11 / 0.14** | 0.96 / 1.84 | 0.38 / 1.13 | 0.58 / 1.63 | 0.12 / 0.14 | 0.16 / 0.20 |
| 8 | 5.79 / 8.22<br>**native 0.16 / 0.22** | 7.57 / 9.52 | 2.36 / 5.96 | 4.20 / 7.17 | 0.19 / 0.28 | 0.26 / 0.44 |
| 32 | 16.72 / 32.98<br>**native 0.39 / 0.80** | 29.91 / 37.45 | 11.78 / 23.01 | 19.51 / 38.03 | 0.42 / 0.98 | 0.93 / 2.41 |
| 64 | 34.56 / 66.13<br>**native 0.79 / 1.60** | 59.92 / 75.25 | 24.08 / 35.26 | 32.82 / 80.31 | 0.83 / 1.81 | 1.92 / 3.72 |
| 128 | 68.11 / 142.91<br>**native 1.55 / 3.24** | 119.27 / 143.37 | 45.23 / 53.61 | 52.18 / 133.47 | 1.64 / 3.95 | 3.93 / 6.34 |
| 256 | 134.69 / 300.43<br>**native 3.05 / 7.02** | 197.12 / 2730.35 | 59.41 / 94.44 | 80.19 / 1116.26 | 3.16 / 9.17 | 7.92 / 11.10 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1.62 / 1.68<br>**native 0.08 / 0.10** | 0.68 / 1.36 | 0.11 / 0.39 | 0.30 / 1.04 | 0.10 / 0.12 | 0.13 / 0.16 |
| 8 | 1.74 / 2.61<br>**native 0.13 / 0.18** | 5.21 / 7.39 | 0.66 / 1.72 | 1.86 / 4.31 | 0.15 / 0.23 | 0.24 / 0.45 |
| 32 | 4.82 / 10.10<br>**native 0.29 / 0.65** | 20.13 / 26.90 | 3.04 / 4.99 | 9.39 / 16.90 | **0.35 / 0.68** | 0.88 / 2.40 |
| 64 | 10.01 / 19.97<br>**native 0.59 / 1.20** | 41.39 / 50.74 | 6.32 / 10.15 | 15.00 / 38.44 | 0.67 / 1.50 | 1.85 / 3.82 |
| 128 | 20.16 / 42.04<br>**native 1.17 / 2.49** | 83.70 / 110.37 | 12.73 / 20.47 | 27.58 / 80.06 | 1.31 / 3.17 | 3.79 / 6.44 |
| 256 | 39.82 / 90.30<br>**native 2.29 / 5.41** | 163.11 / 1048.23 | 25.63 / 41.45 | 51.21 / 134.55 | 2.51 / 7.10 | 7.65 / 11.20 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 7.13 / 8.02<br>native 1.92 / 2.00 | 2.20 / 3.10 | **1.71 / 1.86** | 1.83 / 2.19 | **1.84 / 1.93** | 1.92 / 1.99 |
| 8 | 8.19 / 11.72<br>native 2.03 / 2.99 | 17.43 / 21.14 | 11.59 / 23.74 | 14.25 / 19.09 | 1.92 / 3.36 | **2.05 / 2.24** |
| 32 | 22.61 / 42.32<br>native 5.70 / 12.12 | 69.97 / 81.44 | 52.33 / 103.28 | 62.09 / 98.28 | **5.16 / 11.23** | 5.25 / 17.08 |
| 64 | 46.20 / 86.94<br>native 11.61 / 23.29 | 140.00 / 167.90 | 105.49 / 134.47 | 92.46 / 206.57 | **10.50 / 21.86** | 10.92 / 28.86 |
| 128 | 92.19 / 184.90<br>native 23.34 / 47.47 | 277.40 / 2083.38 | 131.27 / 3758.00 | 143.56 / 3034.56 | 20.64 / 49.28 | **23.31 / 44.31** |
| 256 | 181.83 / 437.60<br>native 46.12 / 106.64 | 309.07 / 7513.16 | 131.72 / 9051.66 | 146.39 / 8474.96 | 39.47 / 115.59 | **48.77 / 81.37** |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.66 / 0.70** | 5.84 / 8.61 | 0.91 / 1.13 | 0.82 / 1.31 | 0.65 / 0.74 | 0.73 / 0.78 |
| 8 | **0.71 / 0.97** | 44.98 / 56.19 | 5.95 / 12.71 | 6.16 / 9.83 | **0.76 / 0.98** | 0.89 / 1.10 |
| 32 | **2.03 / 4.23** | 172.94 / 217.63 | 26.42 / 51.64 | 23.55 / 53.12 | 2.13 / 4.45 | 2.62 / 7.64 |
| 64 | **4.18 / 8.34** | 341.58 / 1675.35 | 54.00 / 73.09 | 46.38 / 106.30 | 4.42 / 9.33 | 5.56 / 12.68 |
| 128 | **8.49 / 17.57** | 480.87 / 8649.69 | 91.39 / 124.30 | 79.56 / 198.83 | 8.73 / 19.36 | 11.48 / 21.51 |
| 256 | **16.88 / 37.42** | 503.09 / 10205.06 | 91.46 / 4856.65 | 92.16 / 4319.88 | 16.66 / 47.07 | **23.08 / 37.99** |

Ranked on p99.

## CPU and memory

What the server used while it was loaded and what it takes to start.

#### Starting and idling

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Spawn to first 200 | **21 ms** | 322 ms | 54 ms | 212 ms | 30 ms | 24 ms |
| Resident at first 200 | 18.9 MB | 95.0 MB | 60.4 MB | 80.1 MB | **5.7 MB** | 10.2 MB |
| Peak resident over the run | 388.8 MB | 669.6 MB | 406.1 MB | 791.9 MB | **101.0 MB** | **97.8 MB** |

#### home: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 49% / 28.6 MB | 101% / 283.0 MB | 137% / 142.2 MB | 91% / 152.3 MB | **68% / 19.0 MB** | 95% / 22.2 MB |
| 8 | 256% / 28.8 MB | 108% / 283.0 MB | 261% / 143.5 MB | 199% / 152.3 MB | **435% / 19.0 MB** | 597% / 22.9 MB |
| 32 | 462% / 30.0 MB | 108% / 283.0 MB | 253% / 143.8 MB | 204% / 153.0 MB | **777% / 19.6 MB** | 557% / 24.3 MB |
| 64 | 500% / 28.6 MB | 108% / 283.2 MB | 224% / 144.5 MB | 206% / 155.5 MB | **823% / 20.8 MB** | 552% / 26.8 MB |
| 128 | 522% / 30.1 MB | 108% / 289.8 MB | 219% / 146.1 MB | 200% / 159.2 MB | **856% / 23.0 MB** | 552% / 32.2 MB |
| 256 | **547% / 33.2 MB** | 108% / 305.1 MB | 232% / 181.1 MB | 199% / 160.5 MB | 891% / 36.9 MB | 553% / 42.5 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### counter: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **49% / 33.4 MB** | 101% / 306.5 MB | 112% / 181.8 MB | 91% / 161.4 MB | 65% / 37.3 MB | 97% / 42.6 MB |
| 8 | **256% / 33.4 MB** | 108% / 306.5 MB | 213% / 181.8 MB | 199% / 161.4 MB | 429% / 37.3 MB | 588% / 42.7 MB |
| 32 | **462% / 33.4 MB** | 108% / 306.5 MB | 220% / 181.8 MB | 204% / 161.5 MB | 762% / 37.4 MB | 543% / 42.7 MB |
| 64 | **505% / 33.5 MB** | 108% / 306.5 MB | 221% / 181.8 MB | 206% / 161.6 MB | 825% / 37.4 MB | 538% / 43.0 MB |
| 128 | **529% / 33.5 MB** | 108% / 306.6 MB | 219% / 181.8 MB | 199% / 161.8 MB | 849% / 37.4 MB | 540% / 43.2 MB |
| 256 | **549% / 33.5 MB** | 108% / 307.3 MB | 229% / 193.6 MB | 198% / 162.4 MB | 846% / 41.1 MB | 543% / 43.8 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### catalog: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **90% / 42.4 MB** | 112% / 504.9 MB | 101% / 253.0 MB | 106% / 225.7 MB | **101% / 42.7 MB** | 111% / 50.6 MB |
| 8 | **656% / 42.5 MB** | 117% / 505.0 MB | 110% / 253.0 MB | 116% / 225.7 MB | **763% / 42.7 MB** | 843% / 52.4 MB |
| 32 | **1122% / 43.5 MB** | 122% / 516.5 MB | 110% / 256.4 MB | 116% / 250.0 MB | **1188% / 43.2 MB** | 911% / 53.2 MB |
| 64 | **1120% / 44.0 MB** | 123% / 526.8 MB | 111% / 268.5 MB | 117% / 276.3 MB | **1193% / 44.0 MB** | 910% / 53.4 MB |
| 128 | **1121% / 44.3 MB** | 125% / 548.9 MB | 112% / 285.1 MB | 118% / 276.4 MB | 1194% / 46.8 MB | 910% / 53.7 MB |
| 256 | **1116% / 44.5 MB** | 126% / 502.3 MB | 112% / 300.8 MB | 117% / 279.0 MB | 1184% / 52.7 MB | 901% / 55.5 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### product: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **79% / 44.8 MB** | 107% / 514.6 MB | 100% / 303.6 MB | 109% / 279.2 MB | 100% / 53.5 MB | 101% / 55.7 MB |
| 8 | **541% / 44.8 MB** | 117% / 514.7 MB | 116% / 303.5 MB | 129% / 279.2 MB | 633% / 53.6 MB | 660% / 55.8 MB |
| 32 | **974% / 44.9 MB** | 119% / 515.0 MB | 116% / 303.6 MB | 129% / 279.2 MB | 936% / 53.7 MB | 654% / 56.0 MB |
| 64 | **992% / 44.9 MB** | 121% / 520.8 MB | 117% / 304.3 MB | 129% / 279.4 MB | 975% / 53.7 MB | 650% / 55.2 MB |
| 128 | **1012% / 51.7 MB** | 124% / 524.6 MB | 118% / 321.3 MB | 131% / 762.5 MB | **1001% / 53.8 MB** | 649% / 55.3 MB |
| 256 | 1015% / 67.9 MB | 127% / 528.0 MB | 119% / 335.0 MB | 130% / 762.5 MB | **990% / 55.7 MB** | **652% / 55.7 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### search: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 82% / 68.1 MB | 105% / 537.6 MB | 101% / 336.2 MB | 107% / 762.5 MB | **100% / 56.0 MB** | **108% / 55.8 MB** |
| 8 | 577% / 68.1 MB | 117% / 537.6 MB | 115% / 336.2 MB | 124% / 762.5 MB | **685% / 56.0 MB** | **712% / 55.8 MB** |
| 32 | 1026% / 68.1 MB | 120% / 537.6 MB | 115% / 336.5 MB | 125% / 762.5 MB | **1070% / 56.3 MB** | **741% / 55.9 MB** |
| 64 | 1047% / 68.1 MB | 121% / 539.4 MB | 115% / 337.4 MB | 125% / 762.6 MB | **1082% / 56.4 MB** | **735% / 56.0 MB** |
| 128 | 1045% / 68.1 MB | 124% / 539.7 MB | 116% / 341.5 MB | 125% / 762.6 MB | **1056% / 56.5 MB** | **739% / 56.0 MB** |
| 256 | 1061% / 70.4 MB | 127% / 543.0 MB | 117% / 352.6 MB | 126% / 762.6 MB | **1078% / 56.7 MB** | **721% / 56.3 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### missing: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 72% / 70.6 MB | 109% / 543.9 MB | 99% / 353.4 MB | 112% / 760.4 MB | 104% / 56.8 MB | **98% / 52.0 MB** |
| 8 | 496% / 70.6 MB | 118% / 543.9 MB | 118% / 353.2 MB | 129% / 760.4 MB | 548% / 56.8 MB | **600% / 52.0 MB** |
| 32 | 870% / 70.6 MB | 122% / 544.3 MB | 118% / 353.3 MB | 129% / 760.6 MB | 795% / 56.8 MB | **565% / 52.1 MB** |
| 64 | 908% / 70.6 MB | 124% / 544.5 MB | 119% / 353.5 MB | 129% / 760.6 MB | 809% / 56.8 MB | **554% / 52.3 MB** |
| 128 | 926% / 70.6 MB | 126% / 545.3 MB | 120% / 353.9 MB | 130% / 760.8 MB | 820% / 56.8 MB | **564% / 52.5 MB** |
| 256 | 957% / 147.6 MB | 127% / 546.8 MB | 121% / 364.0 MB | 131% / 761.3 MB | 828% / 56.8 MB | **568% / 53.1 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### api: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 68% / 147.7 MB | 111% / 512.5 MB | 92% / 367.7 MB | 72% / 761.3 MB | 59% / 57.1 MB | **59% / 53.5 MB** |
| 8 | 437% / 147.7 MB | 113% / 512.4 MB | 119% / 367.7 MB | 100% / 761.3 MB | 334% / 57.1 MB | **421% / 53.6 MB** |
| 32 | 758% / 147.7 MB | 113% / 512.4 MB | 120% / 368.0 MB | 100% / 761.3 MB | 571% / 57.1 MB | **790% / 54.2 MB** |
| 64 | 823% / 147.7 MB | 113% / 513.0 MB | 121% / 368.4 MB | 101% / 761.4 MB | **612% / 57.1 MB** | **766% / 54.5 MB** |
| 128 | 844% / 147.8 MB | 114% / 512.8 MB | 121% / 369.7 MB | 101% / 761.4 MB | **628% / 57.1 MB** | **797% / 55.0 MB** |
| 256 | 869% / 147.8 MB | 114% / 512.7 MB | 122% / 370.7 MB | 102% / 761.4 MB | **647% / 57.2 MB** | 806% / 61.5 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### primes: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 218.4 MB<br>native 87% / 219.3 MB | 103% / 577.6 MB | 92% / 389.3 MB | 105% / 769.8 MB | **102% / 59.1 MB** | 97% / 62.2 MB |
| 8 | 711% / 211.5 MB<br>native 624% / 212.4 MB | 110% / 582.4 MB | 109% / 393.5 MB | 115% / 774.1 MB | **648% / 59.1 MB** | 692% / 62.3 MB |
| 32 | 1266% / 218.5 MB<br>native 1084% / 189.8 MB | 112% / 576.7 MB | 112% / 386.0 MB | 116% / 773.3 MB | **1067% / 59.6 MB** | **746% / 62.3 MB** |
| 64 | 1262% / 218.8 MB<br>native 1085% / 187.6 MB | 115% / 591.7 MB | 112% / 384.0 MB | 117% / 777.5 MB | **1091% / 59.8 MB** | **754% / 62.3 MB** |
| 128 | 1253% / 218.9 MB<br>native 1094% / 187.9 MB | 116% / 582.9 MB | 106% / 392.1 MB | 116% / 764.0 MB | **1094% / 59.9 MB** | **748% / 62.3 MB** |
| 256 | 1266% / 218.9 MB<br>native 1097% / 369.3 MB | 119% / 590.2 MB | 106% / 386.5 MB | 116% / 770.4 MB | **1097% / 61.4 MB** | **756% / 62.5 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 96% / 369.4 MB<br>native 82% / 369.7 MB | 106% / 584.9 MB | 101% / 373.0 MB | 110% / 762.5 MB | **100% / 61.5 MB** | **101% / 62.5 MB** |
| 8 | 717% / 369.4 MB<br>native 572% / 369.7 MB | 118% / 584.9 MB | 113% / 373.0 MB | 128% / 762.5 MB | **643% / 61.5 MB** | **670% / 62.6 MB** |
| 32 | 1271% / 369.4 MB<br>native 1007% / 369.9 MB | 121% / 585.0 MB | 113% / 373.1 MB | 128% / 762.5 MB | **976% / 61.6 MB** | **690% / 62.6 MB** |
| 64 | 1269% / 369.4 MB<br>native 1028% / 370.0 MB | 121% / 585.2 MB | 114% / 373.2 MB | 128% / 762.5 MB | **1026% / 61.7 MB** | **690% / 62.7 MB** |
| 128 | 1263% / 369.4 MB<br>native 1042% / 370.0 MB | 125% / 587.8 MB | 115% / 376.5 MB | 129% / 762.5 MB | **1034% / 62.0 MB** | **685% / 62.8 MB** |
| 256 | 1265% / 369.5 MB<br>native 1046% / 370.1 MB | 127% / 592.6 MB | 116% / 386.8 MB | 129% / 762.5 MB | **1039% / 62.4 MB** | **688% / 63.0 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 208.1 MB<br>native 96% / 208.2 MB | 105% / 592.8 MB | 98% / 388.0 MB | 102% / 762.5 MB | **96% / 62.5 MB** | **96% / 63.6 MB** |
| 8 | 712% / 208.1 MB<br>native 721% / 208.2 MB | 108% / 592.8 MB | 101% / 387.7 MB | 105% / 762.5 MB | **753% / 62.5 MB** | **778% / 63.6 MB** |
| 32 | 1281% / 208.1 MB<br>native 1260% / 208.2 MB | 107% / 593.3 MB | 103% / 388.1 MB | 106% / 762.6 MB | **1292% / 62.6 MB** | **1234% / 63.7 MB** |
| 64 | 1286% / 208.1 MB<br>native 1267% / 208.2 MB | 107% / 594.6 MB | 104% / 388.8 MB | 106% / 762.6 MB | **1297% / 62.6 MB** | **1223% / 63.7 MB** |
| 128 | 1286% / 208.1 MB<br>native 1269% / 208.2 MB | 110% / 594.7 MB | 104% / 389.3 MB | 105% / 762.6 MB | **1291% / 62.7 MB** | **1257% / 63.9 MB** |
| 256 | 1278% / 208.1 MB<br>native 1267% / 208.2 MB | 110% / 594.9 MB | 89% / 389.3 MB | 93% / 762.6 MB | **1260% / 63.0 MB** | **1238% / 64.0 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 90% / 212.7 MB | 111% / 666.0 MB | 98% / 397.7 MB | 106% / 762.6 MB | **96% / 68.4 MB** | 104% / 81.3 MB |
| 8 | 710% / 212.7 MB | 118% / 678.8 MB | 108% / 397.7 MB | 114% / 762.6 MB | **778% / 68.5 MB** | 824% / 81.4 MB |
| 32 | 1191% / 212.9 MB | 119% / 695.0 MB | 110% / 431.4 MB | 116% / 762.6 MB | **1180% / 70.3 MB** | 1117% / 88.0 MB |
| 64 | 1194% / 213.9 MB | 121% / 691.2 MB | 109% / 441.4 MB | 117% / 762.7 MB | **1191% / 72.9 MB** | 1121% / 89.8 MB |
| 128 | 1191% / 214.0 MB | 122% / 709.2 MB | 107% / 441.4 MB | 119% / 762.8 MB | **1216% / 79.8 MB** | 1106% / 94.1 MB |
| 256 | 1189% / 231.2 MB | 112% / 703.1 MB | 108% / 441.4 MB | 118% / 763.2 MB | **1200% / 101.0 MB** | **1113% / 97.8 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

## Build

Wall time of a production build, from nothing compiled and after one page changed.

#### Production build

| Build | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| cold | 45.3 s | 2.8 s | **1.5 s** | 3.4 s | 48.5 s | 56.9 s |
| warm | **0.8 s** | 1.4 s | 1.3 s | 2.6 s | 14.0 s | 13.2 s |

## Size

What a deployment carries and what each page costs a browser.

#### Deploy

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Deploy tree | 25.7 MiB | 44.5 MiB | **1.5 MiB** | 2.4 MiB | 13.3 MiB | 8.6 MiB |
| Files in the tree | 458 | 1,351 | 181 | 168 | **6** | **6** |
| Server binary as built | 22.1 MiB | - | - | - | 12.5 MiB | **7.3 MiB** |
| Server binary stripped | 16.9 MiB | - | - | - | 7.1 MiB | **4.7 MiB** |
| Runtime needed beside it | - | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | - | - |
| Total to deploy | 25.7 MiB | 151.8 MiB | 108.8 MiB | 109.7 MiB | 13.3 MiB | **8.6 MiB** |

The total is the deploy tree, which holds the server binary where there is one, plus the runtime that has to be installed beside it.

#### Document

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 1.9 KiB (5.9 KiB raw) | 1.9 KiB (6.4 KiB raw) | **1.1 KiB (2.7 KiB raw)** | **1.1 KiB (2.3 KiB raw)** | **1.1 KiB (2.2 KiB raw)** | **1.1 KiB (2.5 KiB raw)** |
| counter | 1.7 KiB (5.8 KiB raw) | 1.7 KiB (5.9 KiB raw) | **0.9 KiB (2.2 KiB raw)** | **0.9 KiB (1.9 KiB raw)** | **0.9 KiB (1.8 KiB raw)** | **0.9 KiB (1.9 KiB raw)** |
| catalog | **2.5 KiB (14.4 KiB raw)** | 3.7 KiB (34.2 KiB raw) | **2.4 KiB (15.0 KiB raw)** | 2.9 KiB (15.4 KiB raw) | 2.7 KiB (15.5 KiB raw) | 4.3 KiB (27.9 KiB raw) |
| product | 1.9 KiB (6.5 KiB raw) | 2.1 KiB (9.2 KiB raw) | **1.3 KiB (3.7 KiB raw)** | **1.3 KiB (3.5 KiB raw)** | 1.6 KiB (4.0 KiB raw) | 1.7 KiB (4.9 KiB raw) |
| search | 2.0 KiB (7.7 KiB raw) | 2.4 KiB (12.9 KiB raw) | **1.5 KiB (5.4 KiB raw)** | 1.6 KiB (5.3 KiB raw) | 1.8 KiB (5.7 KiB raw) | 2.1 KiB (8.2 KiB raw) |
| primes | 1.7 KiB (5.7 KiB raw) | 1.8 KiB (6.8 KiB raw) | **1.0 KiB (2.4 KiB raw)** | **1.0 KiB (2.2 KiB raw)** | 1.3 KiB (2.8 KiB raw) | 1.1 KiB (2.6 KiB raw) |
| report | 2.0 KiB (6.7 KiB raw) | 2.4 KiB (10.8 KiB raw) | **1.6 KiB (4.6 KiB raw)** | 1.7 KiB (4.6 KiB raw) | 1.9 KiB (5.2 KiB raw) | 2.2 KiB (7.2 KiB raw) |
| words | 1.8 KiB (6.0 KiB raw) | 2.0 KiB (8.2 KiB raw) | **1.2 KiB (3.1 KiB raw)** | 1.2 KiB (3.0 KiB raw) | 1.4 KiB (3.6 KiB raw) | 1.4 KiB (4.3 KiB raw) |
| table | **4.9 KiB (54.1 KiB raw)** | 13.5 KiB (203.6 KiB raw) | 7.3 KiB (92.7 KiB raw) | 10.8 KiB (91.3 KiB raw) | 8.3 KiB (107.5 KiB raw) | 22.5 KiB (229.0 KiB raw) |

Gzipped, with the raw size beside it.

#### JavaScript

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.9 KiB (88.1 KiB raw) | 69.5 KiB (185.3 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| counter | 40.0 KiB (115.2 KiB raw) | 135.0 KiB (454.5 KiB raw) | 33.7 KiB (87.8 KiB raw) | 69.3 KiB (184.9 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| catalog | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.0 KiB (88.4 KiB raw) | 74.5 KiB (204.5 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| product | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.2 KiB (88.7 KiB raw) | 74.6 KiB (204.7 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| search | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.1 KiB (88.4 KiB raw) | 74.5 KiB (204.4 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| primes | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.5 KiB (87.5 KiB raw) | 74.6 KiB (204.6 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| report | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.8 KiB (88.0 KiB raw) | 74.8 KiB (205.1 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| words | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.5 KiB (87.5 KiB raw) | 74.6 KiB (204.6 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |
| table | 40.0 KiB (115.2 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.7 KiB (87.9 KiB raw) | 74.8 KiB (204.9 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) |

Gzipped, with the raw size beside it.

#### WebAssembly

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| counter | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| catalog | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| product | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| search | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| primes | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| report | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| words | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |
| table | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) |

Gzipped, with the raw size beside it.

Leptos and Dioxus hydrate by loading the whole app compiled to WebAssembly, so a page costs a browser this on top of its JavaScript. The other four hydrate with JavaScript alone and load none.

#### CSS

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| counter | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| catalog | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| product | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| search | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| primes | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| report | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| words | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |
| table | 0.7 KiB (1.3 KiB raw) | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | **0.6 KiB (1.2 KiB raw)** | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) |

Gzipped, with the raw size beside it.

#### Requests to show the page

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 32, then 7 later (79.8 KiB) | 8 | 12 | 6 | **4** | **4** |
| counter | 32, then 7 later (79.8 KiB) | 9 | 12 | 6 | **4** | **4** |
| catalog | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | **4** | **4** |
| product | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | **4** | **4** |
| search | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | **4** | **4** |
| primes | 32, then 7 later (79.8 KiB) | 8 | 10 | 8 | **4** | **4** |
| report | 32, then 7 later (79.8 KiB) | 8 | 11 | 9 | **4** | **4** |
| words | 32, then 7 later (79.8 KiB) | 8 | 10 | 8 | **4** | **4** |
| table | 32, then 7 later (79.8 KiB) | 8 | 11 | 9 | **4** | **4** |

The document, stylesheets, WebAssembly and scripts a browser fetches before the page is whole. Modules reached only through a dynamic `import()` follow as loaded later, gzipped.

## Method

- Every application is the same storefront: 48 products in a module each imports, a landing page and a counter page built once at build time and a catalog, product and search page rendered for each request because each reads the query or a parameter. The data, the compute code and the stylesheet are shared byte for byte, with the data and compute code written once in TypeScript and once in Rust for Leptos and Dioxus. The views are shared as TSX by FSR and Next.js and written once each as Svelte, Vue, Leptos and Dioxus components for those four, with the same markup.
- Three compute pages run a loader that loops and allocates per request, the work a JIT is built for: a sieve of Eratosthenes, an aggregation of generated orders sorted with a comparator and a word count over a built-up text. A table page renders 500 rows from simple data. Each writes a digest of what it computed and a run refuses to start when the applications' digests differ. FSR is measured on the three compute pages a second time with the loader calling the Rust that Leptos and Dioxus run through `#[native]`. That figure is shown under its own.
- FSR is built with `fsr build app` and then `cargo build --release` under `RELEASE_ENV=production`, which compiles each loader the generator can write into the binary, then prerenders its fixed routes. Its render memo (`[cache]`) is off, so every live request renders. Next.js is built with `next build` and served from its `standalone` output with `node server.js`; the live pages set `dynamic = "force-dynamic"`. SvelteKit is built with `vite build` and `@sveltejs/adapter-node` and served with `node build`; each live page computes in a `+page.server.ts` load. Nuxt is built with `nuxt build` and served with `node .output/server/index.mjs`; each live page computes in `useAsyncData`. Both prerender the landing and counter pages. Leptos is built with `cargo leptos build --release` and served by its Axum binary; each live page computes in a server function read through a blocking resource. Every Leptos page renders per request, the landing and counter pages included. Dioxus is built with `dx bundle --platform web --fullstack --release` and served by the server binary it writes; each live page computes in a server function read through `use_server_future`. Every Dioxus page also renders per request.
- A cold build starts from empty output directories with dependencies already downloaded and `node_modules` installed: a fresh `CARGO_TARGET_DIR` and no rustc wrapper for FSR, Leptos and Dioxus, a fresh `distDir` for Next.js and fresh output, build and Vite cache directories for SvelteKit and Nuxt. A warm build touches one page and builds again.
- The deploy tree is what a server needs: for FSR the tree `fsr bundle` writes plus `config/` and the binary as built with debug information; for Next.js the `standalone` directory with its static files copied in; for SvelteKit the adapter's `build/` and `package.json`; for Nuxt `.output/`; for Leptos its `site/` directory and the binary; for Dioxus the `web/` directory `dx` writes, which holds the server binary and `public/`. Next.js, SvelteKit and Nuxt also need Node installed, listed apart.
- Bytes per page are what a browser fetches to show it: the document, its stylesheets, the WebAssembly it preloads and its scripts, following the static imports of module scripts. Modules reached only by a dynamic `import()` are listed apart as loaded later. Sizes are raw and gzipped at the default level.
- Each load cell holds one HTTP/1.1 connection per concurrent client and sends the next request when the last body is read, for a fixed number of seconds. Each repetition starts every server afresh and the applications alternate, their order flipping between repetitions. Every page is checked for its status, its text and its product count before it is loaded. Figures are medians across repetitions.

## Caveats

- The load client runs on the same machine as the server and takes CPU from it.
- FSR, Leptos and Dioxus serve on every core. Next.js, SvelteKit and Nuxt each serve from one Node process; running several behind a balancer is a deployment choice this arena does not make.
