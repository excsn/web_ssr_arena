# Arena results

The same storefront written in SnapFire FSR, Next.js, SvelteKit, Nuxt, Astro, Leptos and Dioxus, built for production and served on one machine: build time, deploy size, bytes per page and requests per second.

Higher is better for requests per second. Lower is better for latency, memory, build time and size. Bold marks the best figure in a row and every figure within 5% of it.

Machine: Apple M4 Pro (14 cores).

- Astro: astro 7.4.12, react 19.3.1
- Dioxus: dioxus 0.7.10
- SnapFire FSR: snapfire_fsr_host 0.26.0
- Leptos: leptos 0.8.23
- Next.js: next 16.4.4, react 19.3.1
- Nuxt: nuxt 4.6.4, vue 3.5.44
- SvelteKit: @sveltejs/kit 3.1.1, svelte 5.57.3

Load: 10 s cells after 3 s of warm-up per page, 4 client threads. Started 2026-11-03T16:53:25Z in high power power mode at load 6.23. ended 2026-11-13T23:28:55Z in high power power mode at load 4.41.

Build: Started 2026-11-14T09:51:41Z in high power power mode at load 16.45. ended 2026-11-14T09:52:28Z in high power power mode at load 20.35 for SnapFire FSR; Started 2026-11-03T16:47:59Z in high power power mode at load 20.43. ended 2026-11-03T16:52:52Z in high power power mode at load 9.25 for Next.js, SvelteKit, Nuxt, Leptos and Dioxus; Started 2026-11-13T22:52:27Z in high power power mode at load 2.91. ended 2026-11-13T22:52:29Z in high power power mode at load 2.91 for Astro.

Size: measured 2026-11-14T09:52:30Z for SnapFire FSR; 2026-11-03T16:53:07Z for Next.js, SvelteKit, Nuxt, Leptos and Dioxus; 2026-11-13T22:53:45Z for Astro.

## Requests per second

Responses read in full per second at each number of concurrent clients.

#### home: `/`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **41,227** | 7,632 | 13,496 | 10,983 | 22,020 | 10,389 | 11,898 |
| 8 | **145,856** | 8,741 | 26,369 | 29,170 | 99,845 | 34,585 | 28,045 |
| 32 | **178,143** | 8,541 | 28,770 | 30,133 | 155,975 | 36,077 | 29,334 |
| 64 | **177,175** | 8,557 | 32,114 | 30,201 | 162,622 | 35,995 | 30,262 |
| 128 | **175,608** | 8,430 | 32,079 | 29,283 | 164,653 | 36,091 | 29,675 |
| 256 | **172,962** | 8,404 | 29,435 | 28,854 | **169,079** | 36,246 | 29,391 |

The landing page: a hero and three features, no data.

#### counter: `/counter`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **40,585** | 7,602 | 14,694 | 10,865 | 23,910 | 11,068 | 11,727 |
| 8 | **146,889** | 8,623 | 30,502 | 28,784 | 102,435 | 34,746 | 27,152 |
| 32 | **177,102** | 8,644 | 31,196 | 29,812 | 157,534 | 36,088 | 28,152 |
| 64 | **175,770** | 8,632 | 31,756 | 29,670 | 163,478 | 36,238 | 28,983 |
| 128 | **175,224** | 8,547 | 32,016 | 29,000 | **167,743** | 36,344 | 28,811 |
| 256 | **172,810** | 8,451 | 29,176 | 28,392 | **168,051** | 36,604 | 28,517 |

A fixed page holding one component that hydrates in the browser.

#### catalog: `/products?sort=price`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **6,930** | 826 | 5,872 | 1,765 | 4,138 | 3,330 | **7,099** |
| 8 | **37,363** | 904 | 6,441 | 2,055 | 23,157 | 14,066 | 8,593 |
| 32 | **57,692** | 881 | 6,487 | 1,999 | 35,330 | 13,006 | 8,519 |
| 64 | **57,869** | 865 | 6,436 | 1,868 | 35,408 | 13,013 | 8,438 |
| 128 | **55,816** | 851 | 6,167 | 1,929 | 35,044 | 13,049 | 8,508 |
| 256 | **55,700** | 830 | 6,156 | 1,916 | 33,814 | 13,002 | 8,341 |

All 48 products, sorted by the query.

#### product: `/products/cedar-lantern`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **15,612** | 1,323 | 11,031 | 3,096 | 12,099 | 8,664 | 9,335 |
| 8 | **72,207** | 1,558 | 13,790 | 3,829 | 55,815 | 30,738 | 12,325 |
| 32 | **121,829** | 1,514 | 13,800 | 3,893 | 93,882 | 32,289 | 12,363 |
| 64 | **123,863** | 1,459 | 13,624 | 3,793 | 99,872 | 32,142 | 12,186 |
| 128 | **126,334** | 1,481 | 13,171 | 3,722 | 102,790 | 32,175 | 12,113 |
| 256 | **126,882** | 1,518 | 12,656 | 3,696 | 102,065 | 32,049 | 11,912 |

One product by its slug, with four related products.

#### search: `/search?q=pack`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **12,409** | 1,291 | 9,678 | 2,732 | 9,099 | 6,815 | 9,044 |
| 8 | **61,589** | 1,482 | 11,317 | 3,379 | 45,691 | 25,539 | 11,770 |
| 32 | **107,952** | 1,472 | 11,520 | 3,344 | 78,166 | 26,985 | 11,762 |
| 64 | **106,255** | 1,402 | 11,440 | 3,246 | 79,932 | 26,811 | 11,608 |
| 128 | **106,906** | 1,412 | 11,146 | 3,239 | 79,564 | 26,927 | 11,534 |
| 256 | **108,515** | 1,414 | 10,714 | 3,169 | 80,449 | 26,017 | 11,337 |

Products whose name or tag matches the query.

#### missing: `/products/nope`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **19,387** | 1,149 | 12,632 | 1,924 | 16,644 | 10,177 | 8,119 |
| 8 | **85,650** | 1,297 | 16,127 | 2,398 | 67,289 | 32,835 | 10,166 |
| 32 | **141,218** | 1,280 | 16,113 | 2,420 | 105,319 | 34,701 | 10,123 |
| 64 | **147,101** | 1,236 | 15,889 | 2,367 | 110,110 | 34,345 | 10,028 |
| 128 | **149,573** | 1,243 | 15,576 | 2,319 | 113,409 | 34,953 | 9,832 |
| 256 | **153,925** | 1,254 | 14,961 | 2,329 | 115,669 | 35,108 | 9,650 |

A slug with no product, answered 404.

#### api: `/api/products`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 21,769 | 6,410 | 17,402 | 22,715 | **30,810** | 18,327 | 10,894 |
| 8 | 105,759 | 6,788 | 25,451 | 40,578 | **127,998** | 79,967 | 15,405 |
| 32 | 162,933 | 6,712 | 25,338 | 42,368 | **172,697** | 149,825 | 15,305 |
| 64 | **165,623** | 6,710 | 25,349 | 41,314 | **171,057** | **163,376** | 15,365 |
| 128 | **167,799** | 6,717 | 24,814 | 39,999 | **169,373** | **170,573** | 15,206 |
| 256 | **171,027** | 6,710 | 23,755 | 39,614 | **168,457** | **171,040** | 14,601 |

The catalog as JSON.

#### primes: `/compute/primes?n=50000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 189<br>**native 8,731** | 963 | 2,435 | 1,498 | 8,218 | 6,203 | 2,314 |
| 8 | 1,257<br>**native 48,463** | 1,061 | 2,768 | 1,835 | 42,043 | 29,294 | 2,542 |
| 32 | 1,811<br>**native 83,749** | 1,062 | 2,564 | 1,742 | 73,409 | 31,566 | 2,481 |
| 64 | 1,640<br>**native 84,823** | 1,049 | 2,531 | 1,720 | 73,763 | 32,048 | 2,494 |
| 128 | 1,763<br>**native 83,503** | 1,047 | 2,754 | 1,815 | 73,737 | 31,923 | 2,503 |
| 256 | 1,787<br>**native 83,779** | 1,054 | 2,766 | 1,815 | 73,489 | 31,952 | 2,519 |

A sieve of Eratosthenes up to the query's n: tight numeric loops over an array.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: `/compute/report?orders=5000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 563<br>**native 12,167** | 1,372 | 8,665 | 3,053 | 10,499 | 7,751 | 8,126 |
| 8 | 4,029<br>**native 61,663** | 1,551 | 10,183 | 3,841 | 51,338 | 30,923 | 10,275 |
| 32 | 6,167<br>**native 109,142** | 1,562 | 10,147 | 3,789 | 88,265 | 32,928 | 10,143 |
| 64 | 6,141<br>**native 110,767** | 1,523 | 10,004 | 3,645 | 90,790 | 33,102 | 10,141 |
| 128 | 6,094<br>**native 109,013** | 1,487 | 9,787 | 3,589 | 92,004 | 32,953 | 10,016 |
| 256 | 5,998<br>**native 106,427** | 1,481 | 9,522 | 3,645 | 92,903 | 32,923 | 9,806 |

Generated orders aggregated per product, sorted by revenue with a comparator and formatted.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: `/compute/words?passes=20`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 133<br>native 549 | 433 | **577** | 542 | 542 | 520 | **591** |
| 8 | 902<br>**native 3,995** | 454 | 601 | 555 | **4,037** | **3,890** | 607 |
| 32 | 1,368<br>**native 6,115** | 450 | 594 | 551 | **5,900** | 5,180 | 605 |
| 64 | 1,356<br>**native 6,023** | 449 | 601 | 545 | **5,913** | 5,057 | 599 |
| 128 | 1,326<br>**native 6,002** | 452 | 591 | 555 | **5,886** | 5,172 | 602 |
| 256 | 1,330<br>**native 5,877** | 450 | 518 | 489 | **5,818** | 5,081 | 601 |

Word frequencies over a built-up text: splitting, lowercasing and counting in an object.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: `/table?rows=500`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,491 | 167 | 1,092 | 1,172 | 1,518 | 1,368 | **3,417** |
| 8 | **10,651** | 172 | 1,159 | 1,287 | **10,311** | 8,872 | 4,079 |
| 32 | **15,713** | 181 | 1,132 | 1,243 | 14,217 | 10,834 | 4,017 |
| 64 | **14,833** | 184 | 1,150 | 1,223 | 14,013 | 10,763 | 3,944 |
| 128 | **14,858** | 186 | 1,157 | 1,233 | **14,137** | 10,686 | 3,903 |
| 256 | **14,761** | 169 | 1,160 | 1,248 | **14,127** | 10,803 | 3,889 |

500 rows rendered from simple data: the cost is the render, not the loader.

## Latency

From sending a request to having read its body, median and 99th percentile.

#### home: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.02 / 0.04** | 0.12 / 0.39 | 0.07 / 0.26 | 0.09 / 0.11 | 0.04 / 0.06 | 0.10 / 0.13 | 0.08 / 0.12 |
| 8 | **0.05 / 0.09** | 0.77 / 1.93 | 0.26 / 1.15 | 0.26 / 0.55 | 0.08 / 0.12 | 0.21 / 0.47 | 0.28 / 0.44 |
| 32 | **0.18 / 0.27** | 3.56 / 7.31 | 1.00 / 2.19 | 1.00 / 1.52 | 0.20 / 0.35 | 0.79 / 2.45 | 1.05 / 1.47 |
| 64 | **0.36 / 0.49** | 7.35 / 10.84 | 1.97 / 2.57 | 2.02 / 2.91 | 0.38 / 0.71 | 1.68 / 3.77 | 2.05 / 2.68 |
| 128 | **0.72 / 0.88** | 14.76 / 22.07 | 4.00 / 4.74 | 4.31 / 5.74 | 0.75 / 1.43 | 3.45 / 6.19 | 4.27 / 5.01 |
| 256 | **1.48 / 1.68** | 29.65 / 44.12 | 8.52 / 11.97 | 8.75 / 11.00 | 1.45 / 2.94 | 6.97 / 10.56 | 8.69 / 9.70 |

Ranked on p99.

#### counter: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.02 / 0.04** | 0.12 / 0.40 | 0.07 / 0.10 | 0.09 / 0.11 | 0.04 / 0.06 | 0.09 / 0.12 | 0.08 / 0.13 |
| 8 | **0.05 / 0.09** | 0.78 / 1.96 | 0.25 / 0.47 | 0.27 / 0.55 | 0.08 / 0.12 | 0.21 / 0.48 | 0.29 / 0.46 |
| 32 | **0.18 / 0.26** | 3.49 / 7.22 | 0.99 / 1.44 | 1.02 / 1.55 | 0.20 / 0.35 | 0.79 / 2.46 | 1.09 / 1.51 |
| 64 | **0.36 / 0.48** | 7.32 / 11.10 | 1.98 / 2.62 | 2.04 / 3.00 | 0.38 / 0.70 | 1.67 / 3.81 | 2.15 / 2.80 |
| 128 | **0.73 / 0.87** | 14.78 / 20.95 | 4.00 / 4.83 | 4.32 / 5.87 | 0.74 / 1.42 | 3.43 / 6.16 | 4.40 / 5.14 |
| 256 | **1.48 / 1.68** | 29.69 / 43.53 | 8.53 / 11.91 | 8.91 / 11.45 | 1.47 / 2.93 | 6.89 / 10.64 | 8.96 / 9.86 |

Ranked on p99.

#### catalog: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.14 / 0.17** | 1.12 / 1.93 | 0.16 / 0.38 | 0.54 / 0.92 | 0.24 / 0.27 | 0.30 / 0.34 | 0.13 / 0.43 |
| 8 | **0.21 / 0.28** | 8.67 / 11.93 | 1.03 / 2.48 | 3.96 / 6.57 | 0.34 / 0.43 | 0.54 / 1.04 | 0.87 / 1.50 |
| 32 | **0.54 / 1.04** | 35.69 / 44.74 | 4.86 / 7.96 | 13.48 / 35.23 | 0.87 / 1.83 | 2.41 / 4.47 | 3.69 / 6.78 |
| 64 | **1.10 / 2.07** | 71.75 / 92.39 | 9.72 / 16.28 | 30.10 / 78.66 | 1.76 / 3.61 | 4.85 / 7.38 | 7.41 / 13.89 |
| 128 | **2.27 / 4.69** | 141.70 / 308.39 | 19.70 / 33.19 | 60.15 / 127.95 | 3.49 / 8.13 | 9.71 / 13.38 | 15.36 / 26.50 |
| 256 | **4.48 / 10.05** | 215.52 / 4341.42 | 38.83 / 61.20 | 75.48 / 810.49 | 6.88 / 20.06 | 19.59 / 25.35 | 30.44 / 53.23 |

Ranked on p99.

#### product: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.06 / 0.08** | 0.69 / 1.47 | 0.09 / 0.36 | 0.29 / 0.96 | 0.08 / 0.10 | 0.11 / 0.14 | 0.10 / 0.47 |
| 8 | **0.11 / 0.16** | 5.09 / 7.68 | 0.47 / 1.33 | 1.84 / 4.29 | 0.14 / 0.21 | 0.24 / 0.48 | 0.60 / 1.20 |
| 32 | **0.25 / 0.57** | 20.57 / 27.78 | 2.15 / 3.95 | 8.95 / 16.77 | 0.33 / 0.61 | 0.90 / 2.51 | 2.44 / 4.53 |
| 64 | **0.51 / 1.02** | 42.86 / 54.46 | 4.66 / 8.00 | 15.12 / 37.05 | 0.62 / 1.18 | 1.91 / 3.93 | 5.18 / 9.56 |
| 128 | **0.99 / 2.09** | 83.64 / 106.71 | 9.33 / 16.52 | 27.02 / 77.04 | 1.17 / 2.77 | 3.89 / 6.49 | 10.90 / 18.47 |
| 256 | **1.94 / 4.59** | 156.84 / 996.46 | 19.27 / 33.15 | 50.65 / 130.54 | 2.29 / 6.44 | 7.89 / 11.54 | 21.88 / 36.79 |

Ranked on p99.

#### search: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.08 / 0.10** | 0.72 / 1.40 | 0.10 / 0.40 | 0.34 / 0.93 | 0.11 / 0.13 | 0.14 / 0.21 | 0.10 / 0.45 |
| 8 | **0.13 / 0.18** | 5.42 / 7.95 | 0.58 / 1.63 | 2.15 / 4.90 | 0.17 / 0.25 | 0.29 / 0.57 | 0.63 / 1.22 |
| 32 | **0.28 / 0.67** | 21.26 / 28.38 | 2.67 / 4.67 | 10.54 / 19.92 | 0.39 / 0.98 | 1.10 / 2.74 | 2.58 / 4.80 |
| 64 | **0.59 / 1.18** | 44.45 / 56.45 | 5.52 / 9.21 | 17.21 / 44.35 | 0.77 / 1.69 | 2.30 / 4.37 | 5.46 / 9.97 |
| 128 | **1.17 / 2.46** | 86.82 / 110.80 | 11.14 / 18.78 | 30.86 / 88.20 | 1.53 / 3.64 | 4.67 / 7.31 | 11.34 / 19.87 |
| 256 | **2.29 / 5.15** | 166.36 / 1203.57 | 22.63 / 38.23 | 56.50 / 156.54 | 2.90 / 8.22 | 9.54 / 13.93 | 23.01 / 37.93 |

Ranked on p99.

#### missing: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.05 / 0.07** | 0.81 / 1.55 | 0.07 / 0.38 | 0.46 / 1.28 | 0.06 / 0.08 | 0.09 / 0.16 | 0.11 / 0.55 |
| 8 | **0.09 / 0.14** | 6.09 / 9.24 | 0.40 / 1.21 | 3.02 / 6.37 | 0.12 / 0.18 | 0.22 / 0.50 | 0.71 / 1.45 |
| 32 | **0.22 / 0.41** | 24.25 / 33.48 | 1.84 / 3.48 | 12.85 / 19.89 | 0.30 / 0.47 | 0.82 / 2.51 | 2.94 / 6.21 |
| 64 | **0.42 / 0.81** | 50.29 / 64.34 | 3.97 / 7.20 | 25.84 / 38.47 | 0.58 / 0.88 | 1.76 / 3.95 | 6.26 / 12.05 |
| 128 | **0.82 / 1.71** | 98.60 / 128.41 | 7.97 / 13.75 | 51.74 / 79.19 | **1.12 / 1.72** | 3.57 / 6.27 | 13.20 / 23.71 |
| 256 | 1.58 / 3.75 | 173.43 / 1745.76 | 16.24 / 28.26 | 95.71 / 134.89 | **2.18 / 3.44** | 7.20 / 10.80 | 25.81 / 49.43 |

Ranked on p99.

#### api: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.05 / 0.06 | 0.14 / 0.53 | 0.05 / 0.11 | 0.04 / 0.06 | **0.03 / 0.05** | 0.05 / 0.07 | 0.09 / 0.20 |
| 8 | 0.07 / 0.12 | 1.08 / 2.35 | 0.25 / 0.77 | 0.16 / 0.48 | **0.06 / 0.10** | 0.10 / 0.14 | 0.48 / 1.01 |
| 32 | 0.19 / 0.36 | 4.70 / 8.66 | 1.16 / 2.81 | 0.69 / 1.56 | **0.18 / 0.29** | 0.21 / 0.37 | 1.97 / 3.71 |
| 64 | 0.37 / 0.72 | 9.26 / 15.31 | 2.35 / 4.56 | 1.44 / 3.08 | **0.37 / 0.55** | 0.38 / 0.72 | 4.01 / 7.65 |
| 128 | 0.73 / 1.48 | 18.32 / 29.94 | 5.02 / 8.61 | 3.07 / 5.06 | **0.75 / 0.99** | 0.73 / 1.36 | 8.30 / 15.45 |
| 256 | 1.42 / 3.08 | 36.50 / 59.72 | 10.37 / 17.35 | 6.34 / 7.99 | **1.51 / 1.82** | 1.44 / 2.70 | 17.28 / 30.99 |

Ranked on p99.

#### primes: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5.20 / 6.07<br>native 0.11 / 0.16 | 0.96 / 1.84 | 0.38 / 1.13 | 0.58 / 1.63 | **0.12 / 0.14** | 0.16 / 0.20 | 0.39 / 1.21 |
| 8 | 6.16 / 9.18<br>**native 0.16 / 0.22** | 7.57 / 9.52 | 2.36 / 5.96 | 4.20 / 7.17 | 0.19 / 0.28 | 0.26 / 0.44 | 2.90 / 4.77 |
| 32 | 17.25 / 33.18<br>**native 0.37 / 0.80** | 29.91 / 37.45 | 11.78 / 23.01 | 19.51 / 38.03 | 0.42 / 0.98 | 0.93 / 2.41 | 12.59 / 21.37 |
| 64 | 38.25 / 75.56<br>**native 0.75 / 1.44** | 59.92 / 75.25 | 24.08 / 35.26 | 32.82 / 80.31 | 0.83 / 1.81 | 1.92 / 3.72 | 24.83 / 46.59 |
| 128 | 71.11 / 144.85<br>**native 1.50 / 3.19** | 119.27 / 143.37 | 45.23 / 53.61 | 52.18 / 133.47 | 1.64 / 3.95 | 3.93 / 6.34 | 49.08 / 89.83 |
| 256 | 138.28 / 308.35<br>**native 2.94 / 6.94** | 197.12 / 2730.35 | 59.41 / 94.44 | 80.19 / 1116.26 | 3.16 / 9.17 | 7.92 / 11.10 | 87.19 / 122.66 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1.77 / 1.89<br>**native 0.08 / 0.11** | 0.68 / 1.36 | 0.11 / 0.39 | 0.30 / 1.04 | **0.10 / 0.12** | 0.13 / 0.16 | 0.11 / 0.49 |
| 8 | 1.89 / 2.83<br>**native 0.13 / 0.18** | 5.21 / 7.39 | 0.66 / 1.72 | 1.86 / 4.31 | 0.15 / 0.23 | 0.24 / 0.45 | 0.73 / 1.37 |
| 32 | 5.10 / 9.34<br>**native 0.28 / 0.65** | 20.13 / 26.90 | 3.04 / 4.99 | 9.39 / 16.90 | **0.35 / 0.68** | 0.88 / 2.40 | 2.99 / 5.51 |
| 64 | 10.47 / 19.41<br>**native 0.57 / 1.15** | 41.39 / 50.74 | 6.32 / 10.15 | 15.00 / 38.44 | 0.67 / 1.50 | 1.85 / 3.82 | 6.25 / 11.13 |
| 128 | 20.83 / 41.42<br>**native 1.14 / 2.53** | 83.70 / 110.37 | 12.73 / 20.47 | 27.58 / 80.06 | 1.31 / 3.17 | 3.79 / 6.44 | 12.97 / 22.74 |
| 256 | 41.44 / 91.42<br>**native 2.30 / 5.60** | 163.11 / 1048.23 | 25.63 / 41.45 | 51.21 / 134.55 | 2.51 / 7.10 | 7.65 / 11.20 | 25.43 / 45.31 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 7.47 / 8.66<br>native 1.81 / 2.06 | 2.20 / 3.10 | **1.71 / 1.86** | 1.83 / 2.19 | **1.84 / 1.93** | 1.92 / 1.99 | **1.68 / 1.90** |
| 8 | 8.54 / 12.22<br>native 1.91 / 2.80 | 17.43 / 21.14 | 11.59 / 23.74 | 14.25 / 19.09 | 1.92 / 3.36 | **2.05 / 2.24** | 13.10 / 16.29 |
| 32 | 23.06 / 40.58<br>**native 5.10 / 10.06** | 69.97 / 81.44 | 52.33 / 103.28 | 62.09 / 98.28 | 5.16 / 11.23 | 5.25 / 17.08 | 53.41 / 89.69 |
| 64 | 46.91 / 85.52<br>**native 10.58 / 20.03** | 140.00 / 167.90 | 105.49 / 134.47 | 92.46 / 206.57 | 10.50 / 21.86 | 10.92 / 28.86 | 105.84 / 179.49 |
| 128 | 95.23 / 190.36<br>**native 21.17 / 43.11** | 277.40 / 2083.38 | 131.27 / 3758.00 | 143.56 / 3034.56 | 20.64 / 49.28 | **23.31 / 44.31** | 169.41 / 2042.43 |
| 256 | 187.08 / 411.13<br>native 42.39 / 93.76 | 309.07 / 7513.16 | 131.72 / 9051.66 | 146.39 / 8474.96 | 39.47 / 115.59 | **48.77 / 81.37** | 184.08 / 8274.36 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.67 / 0.73 | 5.84 / 8.61 | 0.91 / 1.13 | 0.82 / 1.31 | 0.65 / 0.74 | 0.73 / 0.78 | **0.28 / 0.49** |
| 8 | **0.73 / 0.99** | 44.98 / 56.19 | 5.95 / 12.71 | 6.16 / 9.83 | **0.76 / 0.98** | 0.89 / 1.10 | 1.90 / 2.61 |
| 32 | **1.97 / 3.75** | 172.94 / 217.63 | 26.42 / 51.64 | 23.55 / 53.12 | 2.13 / 4.45 | 2.62 / 7.64 | 7.88 / 13.31 |
| 64 | **4.19 / 8.51** | 341.58 / 1675.35 | 54.00 / 73.09 | 46.38 / 106.30 | 4.42 / 9.33 | 5.56 / 12.68 | 16.41 / 31.02 |
| 128 | **8.44 / 17.79** | 480.87 / 8649.69 | 91.39 / 124.30 | 79.56 / 198.83 | 8.73 / 19.36 | 11.48 / 21.51 | 33.04 / 58.77 |
| 256 | **16.73 / 37.64** | 503.09 / 10205.06 | 91.46 / 4856.65 | 92.16 / 4319.88 | 16.66 / 47.07 | **23.08 / 37.99** | 60.96 / 117.08 |

Ranked on p99.

## CPU and memory

What the server used while it was loaded and what it takes to start.

#### Starting and idling

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Spawn to first 200 | **14 ms** | 322 ms | 54 ms | 212 ms | 30 ms | 24 ms | 71 ms |
| Resident at first 200 | 18.1 MB | 95.0 MB | 60.4 MB | 80.1 MB | **5.7 MB** | 10.2 MB | 78.0 MB |
| Peak resident over the run | 135.2 MB | 669.6 MB | 406.1 MB | 791.9 MB | **101.0 MB** | **97.8 MB** | 632.3 MB |

#### home: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 51% / 27.8 MB | 101% / 283.0 MB | 137% / 142.2 MB | 91% / 152.3 MB | **68% / 19.0 MB** | 95% / 22.2 MB | 98% / 125.9 MB |
| 8 | 256% / 27.8 MB | 108% / 283.0 MB | 261% / 143.5 MB | 199% / 152.3 MB | **435% / 19.0 MB** | 597% / 22.9 MB | 197% / 125.9 MB |
| 32 | 436% / 29.0 MB | 108% / 283.0 MB | 253% / 143.8 MB | 204% / 153.0 MB | **777% / 19.6 MB** | 557% / 24.3 MB | 208% / 126.3 MB |
| 64 | 460% / 29.8 MB | 108% / 283.2 MB | 224% / 144.5 MB | 206% / 155.5 MB | **823% / 20.8 MB** | 552% / 26.8 MB | 212% / 126.8 MB |
| 128 | 480% / 29.1 MB | 108% / 289.8 MB | 219% / 146.1 MB | 200% / 159.2 MB | **856% / 23.0 MB** | 552% / 32.2 MB | 212% / 128.4 MB |
| 256 | **496% / 32.2 MB** | 108% / 305.1 MB | 232% / 181.1 MB | 199% / 160.5 MB | 891% / 36.9 MB | 553% / 42.5 MB | 213% / 131.9 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### counter: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **51% / 32.4 MB** | 101% / 306.5 MB | 112% / 181.8 MB | 91% / 161.4 MB | 65% / 37.3 MB | 97% / 42.6 MB | 98% / 135.6 MB |
| 8 | **257% / 32.4 MB** | 108% / 306.5 MB | 213% / 181.8 MB | 199% / 161.4 MB | 429% / 37.3 MB | 588% / 42.7 MB | 195% / 135.6 MB |
| 32 | **437% / 32.5 MB** | 108% / 306.5 MB | 220% / 181.8 MB | 204% / 161.5 MB | 762% / 37.4 MB | 543% / 42.7 MB | 208% / 135.8 MB |
| 64 | **468% / 32.5 MB** | 108% / 306.5 MB | 221% / 181.8 MB | 206% / 161.6 MB | 825% / 37.4 MB | 538% / 43.0 MB | 212% / 134.1 MB |
| 128 | **481% / 32.5 MB** | 108% / 306.6 MB | 219% / 181.8 MB | 199% / 161.8 MB | 849% / 37.4 MB | 540% / 43.2 MB | 211% / 135.8 MB |
| 256 | **498% / 32.5 MB** | 108% / 307.3 MB | 229% / 193.6 MB | 198% / 162.4 MB | 846% / 41.1 MB | 543% / 43.8 MB | 211% / 137.4 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### catalog: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **90% / 40.8 MB** | 112% / 504.9 MB | 101% / 253.0 MB | 106% / 225.7 MB | **101% / 42.7 MB** | 111% / 50.6 MB | 102% / 230.5 MB |
| 8 | **660% / 40.8 MB** | 117% / 505.0 MB | 110% / 253.0 MB | 116% / 225.7 MB | **763% / 42.7 MB** | 843% / 52.4 MB | 118% / 230.7 MB |
| 32 | **1196% / 41.8 MB** | 122% / 516.5 MB | 110% / 256.4 MB | 116% / 250.0 MB | **1188% / 43.2 MB** | 911% / 53.2 MB | 119% / 238.3 MB |
| 64 | **1208% / 42.1 MB** | 123% / 526.8 MB | 111% / 268.5 MB | 117% / 276.3 MB | **1193% / 44.0 MB** | 910% / 53.4 MB | 119% / 261.6 MB |
| 128 | **1169% / 42.3 MB** | 125% / 548.9 MB | 112% / 285.1 MB | 118% / 276.4 MB | 1194% / 46.8 MB | 910% / 53.7 MB | 119% / 267.0 MB |
| 256 | **1196% / 42.5 MB** | 126% / 502.3 MB | 112% / 300.8 MB | 117% / 279.0 MB | 1184% / 52.7 MB | 901% / 55.5 MB | 120% / 292.2 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### product: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **78% / 43.4 MB** | 107% / 514.6 MB | 100% / 303.6 MB | 109% / 279.2 MB | 100% / 53.5 MB | 101% / 55.7 MB | 101% / 293.5 MB |
| 8 | **537% / 43.5 MB** | 117% / 514.7 MB | 116% / 303.5 MB | 129% / 279.2 MB | 633% / 53.6 MB | 660% / 55.8 MB | 123% / 293.5 MB |
| 32 | **1035% / 43.8 MB** | 119% / 515.0 MB | 116% / 303.6 MB | 129% / 279.2 MB | 936% / 53.7 MB | 654% / 56.0 MB | 123% / 293.5 MB |
| 64 | **1046% / 43.9 MB** | 121% / 520.8 MB | 117% / 304.3 MB | 129% / 279.4 MB | 975% / 53.7 MB | 650% / 55.2 MB | 124% / 293.9 MB |
| 128 | **1066% / 44.0 MB** | 124% / 524.6 MB | 118% / 321.3 MB | 131% / 762.5 MB | 1001% / 53.8 MB | 649% / 55.3 MB | 124% / 294.2 MB |
| 256 | **1070% / 44.2 MB** | 127% / 528.0 MB | 119% / 335.0 MB | 130% / 762.5 MB | 990% / 55.7 MB | 652% / 55.7 MB | 125% / 470.5 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### search: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **82% / 44.3 MB** | 105% / 537.6 MB | 101% / 336.2 MB | 107% / 762.5 MB | 100% / 56.0 MB | 108% / 55.8 MB | 102% / 495.9 MB |
| 8 | **575% / 44.3 MB** | 117% / 537.6 MB | 115% / 336.2 MB | 124% / 762.5 MB | 685% / 56.0 MB | 712% / 55.8 MB | 122% / 495.7 MB |
| 32 | **1095% / 44.6 MB** | 120% / 537.6 MB | 115% / 336.5 MB | 125% / 762.5 MB | 1070% / 56.3 MB | 741% / 55.9 MB | 122% / 495.9 MB |
| 64 | **1087% / 44.7 MB** | 121% / 539.4 MB | 115% / 337.4 MB | 125% / 762.6 MB | 1082% / 56.4 MB | 735% / 56.0 MB | 123% / 496.4 MB |
| 128 | **1095% / 44.7 MB** | 124% / 539.7 MB | 116% / 341.5 MB | 125% / 762.6 MB | 1056% / 56.5 MB | 739% / 56.0 MB | 123% / 496.5 MB |
| 256 | **1105% / 52.6 MB** | 127% / 543.0 MB | 117% / 352.6 MB | 126% / 762.6 MB | 1078% / 56.7 MB | 721% / 56.3 MB | 124% / 496.8 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### missing: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **72% / 52.8 MB** | 109% / 543.9 MB | 99% / 353.4 MB | 112% / 760.4 MB | 104% / 56.8 MB | **98% / 52.0 MB** | 108% / 497.4 MB |
| 8 | **489% / 52.8 MB** | 118% / 543.9 MB | 118% / 353.2 MB | 129% / 760.4 MB | 548% / 56.8 MB | **600% / 52.0 MB** | 128% / 497.2 MB |
| 32 | **888% / 52.8 MB** | 122% / 544.3 MB | 118% / 353.3 MB | 129% / 760.6 MB | 795% / 56.8 MB | **565% / 52.1 MB** | 129% / 497.4 MB |
| 64 | **929% / 52.8 MB** | 124% / 544.5 MB | 119% / 353.5 MB | 129% / 760.6 MB | 809% / 56.8 MB | **554% / 52.3 MB** | 130% / 565.5 MB |
| 128 | **953% / 52.8 MB** | 126% / 545.3 MB | 120% / 353.9 MB | 130% / 760.8 MB | 820% / 56.8 MB | **564% / 52.5 MB** | 130% / 565.7 MB |
| 256 | 980% / 56.0 MB | 127% / 546.8 MB | 121% / 364.0 MB | 131% / 761.3 MB | 828% / 56.8 MB | **568% / 53.1 MB** | 130% / 565.6 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### api: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 70% / 56.2 MB | 111% / 512.5 MB | 92% / 367.7 MB | 72% / 761.3 MB | 59% / 57.1 MB | **59% / 53.5 MB** | 97% / 568.2 MB |
| 8 | **440% / 56.2 MB** | 113% / 512.4 MB | 119% / 367.7 MB | 100% / 761.3 MB | 334% / 57.1 MB | **421% / 53.6 MB** | 122% / 568.2 MB |
| 32 | **763% / 56.3 MB** | 113% / 512.4 MB | 120% / 368.0 MB | 100% / 761.3 MB | 571% / 57.1 MB | **790% / 54.2 MB** | 123% / 568.4 MB |
| 64 | **846% / 56.3 MB** | 113% / 513.0 MB | 121% / 368.4 MB | 101% / 761.4 MB | **612% / 57.1 MB** | **766% / 54.5 MB** | 123% / 568.9 MB |
| 128 | **854% / 56.3 MB** | 114% / 512.8 MB | 121% / 369.7 MB | 101% / 761.4 MB | **628% / 57.1 MB** | **797% / 55.0 MB** | 124% / 569.4 MB |
| 256 | **877% / 56.4 MB** | 114% / 512.7 MB | 122% / 370.7 MB | 102% / 761.4 MB | **647% / 57.2 MB** | 806% / 61.5 MB | 125% / 570.1 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### primes: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 126.3 MB<br>native 87% / 129.0 MB | 103% / 577.6 MB | 92% / 389.3 MB | 105% / 769.8 MB | **102% / 59.1 MB** | 97% / 62.2 MB | 95% / 602.5 MB |
| 8 | 712% / 126.3 MB<br>native 618% / 115.2 MB | 110% / 582.4 MB | 109% / 393.5 MB | 115% / 774.1 MB | **648% / 59.1 MB** | 692% / 62.3 MB | 111% / 594.8 MB |
| 32 | 1274% / 130.6 MB<br>native 1137% / 99.1 MB | 112% / 576.7 MB | 112% / 386.0 MB | 116% / 773.3 MB | **1067% / 59.6 MB** | **746% / 62.3 MB** | 112% / 586.0 MB |
| 64 | 1162% / 130.6 MB<br>native 1154% / 99.1 MB | 115% / 591.7 MB | 112% / 384.0 MB | 117% / 777.5 MB | **1091% / 59.8 MB** | **754% / 62.3 MB** | 112% / 604.5 MB |
| 128 | 1265% / 130.6 MB<br>native 1137% / 99.2 MB | 116% / 582.9 MB | 106% / 392.1 MB | 116% / 764.0 MB | **1094% / 59.9 MB** | **748% / 62.3 MB** | 112% / 589.7 MB |
| 256 | 1291% / 130.7 MB<br>native 1140% / 99.4 MB | 119% / 590.2 MB | 106% / 386.5 MB | 116% / 770.4 MB | **1097% / 61.4 MB** | **756% / 62.5 MB** | 112% / 595.1 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 96% / 99.4 MB<br>native 82% / 99.7 MB | 106% / 584.9 MB | 101% / 373.0 MB | 110% / 762.5 MB | **100% / 61.5 MB** | **101% / 62.5 MB** | 102% / 571.6 MB |
| 8 | 719% / 99.4 MB<br>native 573% / 99.7 MB | 118% / 584.9 MB | 113% / 373.0 MB | 128% / 762.5 MB | **643% / 61.5 MB** | **670% / 62.6 MB** | 119% / 571.5 MB |
| 32 | 1323% / 99.5 MB<br>native 1064% / 99.9 MB | 121% / 585.0 MB | 113% / 373.1 MB | 128% / 762.5 MB | **976% / 61.6 MB** | **690% / 62.6 MB** | 120% / 571.5 MB |
| 64 | 1324% / 99.5 MB<br>native 1077% / 100.1 MB | 121% / 585.2 MB | 114% / 373.2 MB | 128% / 762.5 MB | **1026% / 61.7 MB** | **690% / 62.7 MB** | 120% / 571.5 MB |
| 128 | 1317% / 99.5 MB<br>native 1070% / 100.1 MB | 125% / 587.8 MB | 115% / 376.5 MB | 129% / 762.5 MB | **1034% / 62.0 MB** | **685% / 62.8 MB** | 121% / 571.5 MB |
| 256 | 1278% / 99.6 MB<br>native 1045% / 103.3 MB | 127% / 592.6 MB | 116% / 386.8 MB | 129% / 762.5 MB | **1039% / 62.4 MB** | **688% / 63.0 MB** | 121% / 571.8 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 103.6 MB<br>native 96% / 103.9 MB | 105% / 592.8 MB | 98% / 388.0 MB | 102% / 762.5 MB | **96% / 62.5 MB** | **96% / 63.6 MB** | 98% / 571.4 MB |
| 8 | 718% / 103.7 MB<br>native 719% / 103.9 MB | 108% / 592.8 MB | 101% / 387.7 MB | 105% / 762.5 MB | **753% / 62.5 MB** | **778% / 63.6 MB** | 102% / 571.4 MB |
| 32 | 1319% / 103.8 MB<br>native 1327% / 103.9 MB | 107% / 593.3 MB | 103% / 388.1 MB | 106% / 762.6 MB | **1292% / 62.6 MB** | **1234% / 63.7 MB** | 103% / 571.4 MB |
| 64 | 1328% / 103.8 MB<br>native 1327% / 103.9 MB | 107% / 594.6 MB | 104% / 388.8 MB | 106% / 762.6 MB | **1297% / 62.6 MB** | **1223% / 63.7 MB** | 103% / 571.4 MB |
| 128 | 1303% / 103.8 MB<br>native 1323% / 103.9 MB | 110% / 594.7 MB | 104% / 389.3 MB | 105% / 762.6 MB | **1291% / 62.7 MB** | **1257% / 63.9 MB** | 103% / 571.4 MB |
| 256 | 1315% / 103.8 MB<br>native 1282% / 103.9 MB | 110% / 594.9 MB | 89% / 389.3 MB | 93% / 762.6 MB | **1260% / 63.0 MB** | **1238% / 64.0 MB** | 103% / 571.5 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 91% / 108.5 MB | 111% / 666.0 MB | 98% / 397.7 MB | 106% / 762.6 MB | **96% / 68.4 MB** | 104% / 81.3 MB | 93% / 590.2 MB |
| 8 | 708% / 108.5 MB | 118% / 678.8 MB | 108% / 397.7 MB | 114% / 762.6 MB | **778% / 68.5 MB** | 824% / 81.4 MB | 109% / 591.2 MB |
| 32 | 1251% / 108.9 MB | 119% / 695.0 MB | 110% / 431.4 MB | 116% / 762.6 MB | **1180% / 70.3 MB** | 1117% / 88.0 MB | 111% / 594.0 MB |
| 64 | 1237% / 108.9 MB | 121% / 691.2 MB | 109% / 441.4 MB | 117% / 762.7 MB | **1191% / 72.9 MB** | 1121% / 89.8 MB | 114% / 594.2 MB |
| 128 | 1245% / 109.2 MB | 122% / 709.2 MB | 107% / 441.4 MB | 119% / 762.8 MB | **1216% / 79.8 MB** | 1106% / 94.1 MB | 114% / 594.0 MB |
| 256 | 1235% / 109.3 MB | 112% / 703.1 MB | 108% / 441.4 MB | 118% / 763.2 MB | **1200% / 101.0 MB** | **1113% / 97.8 MB** | 115% / 594.0 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

## Build

Wall time of a production build, from nothing compiled and after one page changed.

#### Production build

| Build | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| cold | 46.1 s | 2.8 s | 1.5 s | 3.4 s | 48.5 s | 56.9 s | **0.8 s** |
| warm | **0.4 s** | 1.4 s | 1.3 s | 2.6 s | 14.0 s | 13.2 s | 0.5 s |

## Size

What a deployment carries and what each page costs a browser.

#### Deploy

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Deploy tree | 27.7 MiB | 44.5 MiB | **1.5 MiB** | 2.4 MiB | 13.3 MiB | 8.6 MiB | 2.6 MiB |
| Files in the tree | 461 | 1,351 | 181 | 168 | **6** | **6** | 36 |
| Server binary as built | 23.2 MiB | - | - | - | 12.5 MiB | **7.3 MiB** | - |
| Server binary stripped | 17.1 MiB | - | - | - | 7.1 MiB | **4.7 MiB** | - |
| Runtime needed beside it | - | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | - | - | Node v23.9.0, 107.3 MiB |
| Total to deploy | 27.7 MiB | 151.8 MiB | 108.8 MiB | 109.7 MiB | 13.3 MiB | **8.6 MiB** | 109.9 MiB |

The total is the deploy tree, which holds the server binary where there is one, plus the runtime that has to be installed beside it.

#### Document

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 1.9 KiB (5.9 KiB raw) | 1.9 KiB (6.4 KiB raw) | **1.1 KiB (2.7 KiB raw)** | **1.1 KiB (2.3 KiB raw)** | **1.1 KiB (2.2 KiB raw)** | **1.1 KiB (2.5 KiB raw)** | **1.1 KiB (2.2 KiB raw)** |
| counter | 1.7 KiB (5.8 KiB raw) | 1.7 KiB (5.9 KiB raw) | **0.9 KiB (2.2 KiB raw)** | **0.9 KiB (1.9 KiB raw)** | **0.9 KiB (1.8 KiB raw)** | **0.9 KiB (1.9 KiB raw)** | 3.0 KiB (7.1 KiB raw) |
| catalog | 2.5 KiB (14.4 KiB raw) | 3.7 KiB (34.2 KiB raw) | 2.4 KiB (15.0 KiB raw) | 2.9 KiB (15.4 KiB raw) | 2.7 KiB (15.5 KiB raw) | 4.3 KiB (27.9 KiB raw) | **1.7 KiB (9.6 KiB raw)** |
| product | 1.9 KiB (6.5 KiB raw) | 2.1 KiB (9.2 KiB raw) | 1.3 KiB (3.7 KiB raw) | 1.3 KiB (3.5 KiB raw) | 1.6 KiB (4.0 KiB raw) | 1.7 KiB (4.9 KiB raw) | **1.1 KiB (2.6 KiB raw)** |
| search | 2.0 KiB (7.7 KiB raw) | 2.4 KiB (12.9 KiB raw) | 1.5 KiB (5.4 KiB raw) | 1.6 KiB (5.3 KiB raw) | 1.8 KiB (5.7 KiB raw) | 2.1 KiB (8.2 KiB raw) | **1.2 KiB (3.7 KiB raw)** |
| primes | 1.7 KiB (5.7 KiB raw) | 1.8 KiB (6.8 KiB raw) | **1.0 KiB (2.4 KiB raw)** | **1.0 KiB (2.2 KiB raw)** | 1.3 KiB (2.8 KiB raw) | 1.1 KiB (2.6 KiB raw) | **0.9 KiB (1.9 KiB raw)** |
| report | 2.0 KiB (6.7 KiB raw) | 2.4 KiB (10.8 KiB raw) | 1.6 KiB (4.6 KiB raw) | 1.7 KiB (4.6 KiB raw) | 1.9 KiB (5.2 KiB raw) | 2.2 KiB (7.2 KiB raw) | **1.3 KiB (3.0 KiB raw)** |
| words | 1.8 KiB (6.0 KiB raw) | 2.0 KiB (8.2 KiB raw) | 1.2 KiB (3.1 KiB raw) | 1.2 KiB (3.0 KiB raw) | 1.4 KiB (3.6 KiB raw) | 1.4 KiB (4.3 KiB raw) | **1.0 KiB (2.2 KiB raw)** |
| table | 4.9 KiB (54.1 KiB raw) | 13.5 KiB (203.6 KiB raw) | 7.3 KiB (92.7 KiB raw) | 10.8 KiB (91.3 KiB raw) | 8.3 KiB (107.5 KiB raw) | 22.5 KiB (229.0 KiB raw) | **4.1 KiB (50.3 KiB raw)** |

Gzipped, with the raw size beside it.

#### JavaScript

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.9 KiB (88.1 KiB raw) | 69.5 KiB (185.3 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| counter | 40.2 KiB (115.8 KiB raw) | 135.0 KiB (454.5 KiB raw) | 33.7 KiB (87.8 KiB raw) | 69.3 KiB (184.9 KiB raw) | **6.7 KiB (23.0 KiB raw)** | 15.8 KiB (67.2 KiB raw) | 67.6 KiB (216.2 KiB raw) |
| catalog | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.0 KiB (88.4 KiB raw) | 74.5 KiB (204.5 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| product | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.2 KiB (88.7 KiB raw) | 74.6 KiB (204.7 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| search | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 34.1 KiB (88.4 KiB raw) | 74.5 KiB (204.4 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| primes | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.5 KiB (87.5 KiB raw) | 74.6 KiB (204.6 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| report | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.8 KiB (88.0 KiB raw) | 74.8 KiB (205.1 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| words | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.5 KiB (87.5 KiB raw) | 74.6 KiB (204.6 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| table | 40.2 KiB (115.8 KiB raw) | 134.7 KiB (454.0 KiB raw) | 33.7 KiB (87.9 KiB raw) | 74.8 KiB (204.9 KiB raw) | 6.7 KiB (23.0 KiB raw) | 15.8 KiB (67.2 KiB raw) | **0.0 KiB (0.0 KiB raw)** |

Gzipped, with the raw size beside it.

#### WebAssembly

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| home | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| counter | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| catalog | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| product | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| search | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| primes | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| report | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| words | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |
| table | - | - | - | - | **257.1 KiB (721.9 KiB raw)** | 483.2 KiB (1.3 MiB raw) | - |

Gzipped, with the raw size beside it.

Leptos and Dioxus hydrate by loading the whole app compiled to WebAssembly, so a page costs a browser this on top of its JavaScript. The other five hydrate with JavaScript alone and load none.

#### CSS

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| counter | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| catalog | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| product | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| search | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| primes | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| report | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| words | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |
| table | 0.7 KiB (1.3 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.2 KiB raw) | 0.6 KiB (1.3 KiB raw) | 0.6 KiB (1.3 KiB raw) | **0.0 KiB (0.0 KiB raw)** |

Gzipped, with the raw size beside it.

#### Requests to show the page

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus | Astro |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 32, then 7 later (79.8 KiB) | 8 | 12 | 6 | 4 | 4 | **1** |
| counter | 32, then 7 later (79.8 KiB) | 9 | 12 | 6 | **4** | **4** | **4** |
| catalog | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | 4 | 4 | **1** |
| product | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | 4 | 4 | **1** |
| search | 32, then 7 later (79.8 KiB) | 8 | 12 | 9 | 4 | 4 | **1** |
| primes | 32, then 7 later (79.8 KiB) | 8 | 10 | 8 | 4 | 4 | **1** |
| report | 32, then 7 later (79.8 KiB) | 8 | 11 | 9 | 4 | 4 | **1** |
| words | 32, then 7 later (79.8 KiB) | 8 | 10 | 8 | 4 | 4 | **1** |
| table | 32, then 7 later (79.8 KiB) | 8 | 11 | 9 | 4 | 4 | **1** |

The document, stylesheets, WebAssembly and scripts a browser fetches before the page is whole. Modules reached only through a dynamic `import()` follow as loaded later, gzipped.

## Method

- Every application is the same storefront: 48 products in a module each imports, a landing page and a counter page built once at build time and a catalog, product and search page rendered for each request because each reads the query or a parameter. The data, the compute code and the stylesheet are shared byte for byte, with the data and compute code written once in TypeScript and once in Rust for Leptos and Dioxus. The views are shared as TSX by FSR and Next.js and written once each as Svelte, Vue, Leptos and Dioxus components for those four, with the same markup.
- Three compute pages run a loader that loops and allocates per request, the work a JIT is built for: a sieve of Eratosthenes, an aggregation of generated orders sorted with a comparator and a word count over a built-up text. A table page renders 500 rows from simple data. Each writes a digest of what it computed and a run refuses to start when the applications' digests differ. FSR is measured on the three compute pages a second time with the loader calling the Rust that Leptos and Dioxus run through `#[native]`. That figure is shown under its own.
- FSR is built with `fsr build app` and then `cargo build --release` under `RELEASE_ENV=production`, which compiles each loader the generator can write into the binary, then prerenders its fixed routes. Its render memo (`[cache]`) is off, so every live request renders. Next.js is built with `next build` and served from its `standalone` output with `node server.js`; the live pages set `dynamic = "force-dynamic"`. SvelteKit is built with `vite build` and `@sveltejs/adapter-node` and served with `node build`; each live page computes in a `+page.server.ts` load. Nuxt is built with `nuxt build` and served with `node .output/server/index.mjs`; each live page computes in `useAsyncData`. Astro is built with `astro build` and `@astrojs/node` in standalone mode and served with `node dist/server/entry.mjs`; each live page computes in its frontmatter. The counter is a React island with `client:load`. All three prerender the landing and counter pages. Leptos is built with `cargo leptos build --release` and served by its Axum binary; each live page computes in a server function read through a blocking resource. Every Leptos page renders per request, the landing and counter pages included. Dioxus is built with `dx bundle --platform web --fullstack --release` and served by the server binary it writes; each live page computes in a server function read through `use_server_future`. Every Dioxus page also renders per request.
- A cold build starts from empty output directories with dependencies already downloaded and `node_modules` installed: a fresh `CARGO_TARGET_DIR` and no rustc wrapper for FSR, Leptos and Dioxus, a fresh `distDir` for Next.js and fresh output, build and Vite cache directories for SvelteKit, Nuxt and Astro. A warm build touches one page and builds again.
- The deploy tree is what a server needs: for FSR the tree `fsr bundle` writes plus `config/` and the binary as built with debug information; for Next.js the `standalone` directory with its static files copied in; for SvelteKit the adapter's `build/` and `package.json`; for Nuxt `.output/`; for Astro `dist/`; for Leptos its `site/` directory and the binary; for Dioxus the `web/` directory `dx` writes, which holds the server binary and `public/`. Next.js, SvelteKit, Nuxt and Astro also need Node installed, listed apart.
- Bytes per page are what a browser fetches to show it: the document, its stylesheets, the WebAssembly it preloads and its scripts, following the static imports of module scripts. Modules reached only by a dynamic `import()` are listed apart as loaded later. Sizes are raw and gzipped at the default level.
- Each load cell holds one HTTP/1.1 connection per concurrent client and sends the next request when the last body is read, for a fixed number of seconds. Each repetition starts every server afresh and the applications alternate, their order flipping between repetitions. Every page is checked for its status, its text and its product count before it is loaded. Figures are medians across repetitions.

## Caveats

- The load client runs on the same machine as the server and takes CPU from it.
- FSR, Leptos and Dioxus serve on every core. Next.js, SvelteKit, Nuxt and Astro each serve from one Node process; running several behind a balancer is a deployment choice this arena does not make.
