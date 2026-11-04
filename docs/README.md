# Arena results

The same storefront written in SnapFire FSR, Next.js, SvelteKit, Nuxt, Leptos and Dioxus, built for production and served on one machine: build time, deploy size, bytes per page and requests per second.

Higher is better for requests per second. Lower is better for latency, memory, build time and size.

Machine: Apple M4 Pro (14 cores).

- Dioxus: dioxus 0.7.10
- SnapFire FSR: snapfire_fsr_host 0.22.0 at 8b6b64eb+dirty
- Leptos: leptos 0.8.23
- Next.js: next 16.4.4, react 19.3.1
- Nuxt: nuxt 4.6.4, vue 3.5.44
- SvelteKit: @sveltejs/kit 3.1.1, svelte 5.57.3

Load: 10 s cells after 3 s of warm-up per page, 3 repetitions, 4 client threads. Started 2026-11-03T16:53:25Z in high power power mode at load 6.23. ended 2026-11-04T06:40:00Z in high power power mode at load 14.4.

Partial: 948 of 1242 cells measured, so a figure may be the median of fewer than 3 repetitions. Next.js repetition 3 has 36 of 66, SvelteKit repetition 3 has 0 of 66, Nuxt repetition 3 has 0 of 66, Leptos repetition 3 has 0 of 66, Dioxus repetition 3 has 0 of 66.

Build: Started 2026-11-03T16:47:59Z in high power power mode at load 20.43. ended 2026-11-03T16:52:52Z in high power power mode at load 9.25.

Size: measured 2026-11-03T16:53:07Z.

## Requests per second

Responses read in full per second at each number of concurrent clients.

#### home: `/`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **38,547** | 7,632 | 13,418 | 10,942 | 22,542 | 10,271 |
| 8 | **143,788** | 8,741 | 25,877 | 28,848 | 99,858 | 34,555 |
| 32 | **178,348** | 8,541 | 29,655 | 29,617 | 157,123 | 36,250 |
| 64 | **178,409** | 8,557 | 31,929 | 30,388 | 163,742 | 36,149 |
| 128 | **171,576** | 8,430 | 31,835 | 29,408 | 166,700 | 36,085 |
| 256 | **173,318** | 8,404 | 28,895 | 29,082 | 170,186 | 35,905 |

The landing page: a hero and three features, no data.

#### counter: `/counter`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **38,848** | 7,602 | 14,515 | 10,842 | 23,671 | 10,900 |
| 8 | **141,952** | 8,623 | 30,362 | 28,999 | 102,255 | 34,668 |
| 32 | **176,858** | 8,644 | 31,326 | 29,958 | 159,105 | 35,877 |
| 64 | **177,289** | 8,632 | 31,403 | 29,872 | 164,786 | 36,039 |
| 128 | **175,575** | 8,547 | 31,857 | 28,654 | 168,434 | 36,276 |
| 256 | **173,267** | 8,451 | 29,120 | 28,231 | 167,438 | 36,347 |

A fixed page holding one component that hydrates in the browser.

#### catalog: `/products?sort=price`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **6,829** | 826 | 5,849 | 1,751 | 4,127 | 3,309 |
| 8 | **36,733** | 904 | 6,400 | 2,006 | 22,857 | 14,001 |
| 32 | **53,397** | 881 | 6,421 | 1,947 | 34,865 | 12,940 |
| 64 | **54,527** | 865 | 6,345 | 1,842 | 34,740 | 12,819 |
| 128 | **53,208** | 851 | 6,092 | 1,902 | 34,111 | 12,798 |
| 256 | **55,909** | 830 | 6,014 | 1,899 | 33,155 | 12,857 |

All 48 products, sorted by the query.

#### product: `/products/cedar-lantern`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **14,499** | 1,323 | 11,032 | 3,077 | 11,847 | 8,577 |
| 8 | **69,435** | 1,558 | 13,855 | 3,769 | 55,760 | 29,857 |
| 32 | **121,236** | 1,514 | 13,877 | 3,826 | 93,668 | 32,003 |
| 64 | **120,803** | 1,459 | 13,665 | 3,746 | 98,245 | 31,359 |
| 128 | **121,024** | 1,481 | 13,245 | 3,686 | 99,822 | 31,471 |
| 256 | **122,888** | 1,518 | 12,601 | 3,695 | 99,405 | 31,790 |

One product by its slug, with four related products.

#### search: `/search?q=pack`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **12,050** | 1,291 | 9,395 | 2,719 | 8,931 | 6,724 |
| 8 | **60,308** | 1,482 | 11,019 | 3,340 | 45,713 | 25,488 |
| 32 | **99,800** | 1,472 | 10,940 | 3,290 | 77,756 | 26,219 |
| 64 | **99,134** | 1,402 | 11,188 | 3,191 | 79,451 | 26,432 |
| 128 | **102,656** | 1,412 | 10,664 | 3,170 | 79,384 | 26,772 |
| 256 | **101,155** | 1,414 | 10,340 | 3,168 | 80,424 | 25,906 |

Products whose name or tag matches the query.

#### missing: `/products/nope`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **18,517** | 1,149 | 12,635 | 1,906 | 16,529 | 10,030 |
| 8 | **82,344** | 1,297 | 16,408 | 2,374 | 67,676 | 32,672 |
| 32 | **135,494** | 1,280 | 16,298 | 2,323 | 105,131 | 34,608 |
| 64 | **139,402** | 1,236 | 16,112 | 2,356 | 110,060 | 34,226 |
| 128 | **145,749** | 1,243 | 15,562 | 2,295 | 113,222 | 34,661 |
| 256 | **148,733** | 1,254 | 15,054 | 2,306 | 115,684 | 34,624 |

A slug with no product, answered 404.

#### api: `/api/products`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 22,639 | 6,562 | 17,470 | 22,477 | **30,619** | 18,016 |
| 8 | 105,455 | 6,845 | 25,547 | 39,358 | **127,580** | 78,282 |
| 32 | 161,956 | 6,848 | 25,541 | 40,762 | **173,385** | 148,338 |
| 64 | 164,536 | 6,747 | 25,356 | 40,116 | **171,829** | 164,102 |
| 128 | 164,459 | 6,807 | 24,934 | 39,248 | **171,524** | 170,700 |
| 256 | 166,249 | 6,667 | 23,575 | 38,337 | 169,814 | **172,321** |

The catalog as JSON.

#### primes: `/compute/primes?n=50000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 91<br>native 7,899 | 991 | 2,326 | 1,489 | **8,199** | 6,036 |
| 8 | 608<br>**native 44,162** | 1,076 | 2,640 | 1,733 | 42,038 | 28,927 |
| 32 | 834<br>native 72,599 | 1,038 | 2,629 | 1,681 | **72,746** | 31,787 |
| 64 | 859<br>native 71,014 | 1,025 | 2,617 | 1,664 | **74,287** | 32,125 |
| 128 | 870<br>native 72,513 | 1,031 | 2,719 | 1,710 | **74,425** | 30,810 |
| 256 | 864<br>native 73,044 | 1,005 | 2,716 | 1,716 | **73,294** | 31,205 |

A sieve of Eratosthenes up to the query's n: tight numeric loops over an array.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: `/compute/report?orders=5000`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 472<br>**native 10,811** | 1,351 | 8,526 | 3,031 | 10,491 | 7,608 |
| 8 | 3,339<br>**native 55,759** | 1,545 | 10,078 | 3,770 | 51,196 | 30,899 |
| 32 | 4,979<br>**native 92,141** | 1,530 | 10,060 | 3,752 | 88,614 | 32,746 |
| 64 | 4,878<br>**native 97,380** | 1,484 | 9,918 | 3,644 | 90,164 | 32,650 |
| 128 | 4,818<br>**native 98,020** | 1,448 | 9,750 | 3,551 | 91,292 | 32,740 |
| 256 | 4,866<br>**native 98,589** | 1,453 | 9,344 | 3,626 | 92,717 | 32,801 |

Generated orders aggregated per product, sorted by revenue with a comparator and formatted.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: `/compute/words?passes=20`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 129<br>native 512 | 435 | **575** | 529 | 545 | 515 |
| 8 | 886<br>native 3,726 | 448 | 600 | 548 | **4,044** | 3,879 |
| 32 | 1,288<br>native 5,447 | 439 | 592 | 542 | **5,874** | 5,218 |
| 64 | 1,272<br>native 5,127 | 436 | 595 | 543 | **5,852** | 5,154 |
| 128 | 1,258<br>native 5,077 | 437 | 587 | 550 | **5,811** | 5,208 |
| 256 | 1,248<br>native 5,149 | 432 | 541 | 483 | **5,691** | 5,144 |

Word frequencies over a built-up text: splitting, lowercasing and counting in an object.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: `/table?rows=500`

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,203 | 176 | 1,057 | 1,130 | **1,474** | 1,366 |
| 8 | 8,504 | 175 | 1,134 | 1,258 | **10,296** | 8,884 |
| 32 | 12,016 | 169 | 1,110 | 1,200 | **13,769** | 10,933 |
| 64 | 11,397 | 178 | 1,140 | 1,214 | **13,817** | 10,838 |
| 128 | 11,182 | 178 | 1,153 | 1,236 | **14,080** | 10,722 |
| 256 | 10,814 | 163 | 1,155 | 1,237 | **14,043** | 10,796 |

500 rows rendered from simple data: the cost is the render, not the loader.

## Latency

From sending a request to having read its body, median and 99th percentile.

#### home: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.03 / 0.04** | 0.12 / 0.39 | 0.07 / 0.26 | 0.09 / 0.15 | 0.04 / 0.06 | 0.10 / 0.14 |
| 8 | **0.05 / 0.09** | 0.77 / 1.93 | 0.26 / 1.17 | 0.26 / 0.55 | 0.08 / 0.13 | 0.21 / 0.47 |
| 32 | **0.18 / 0.27** | 3.56 / 7.31 | 1.00 / 1.97 | 1.02 / 1.61 | 0.20 / 0.36 | 0.78 / 2.44 |
| 64 | **0.35 / 0.48** | 7.35 / 10.84 | 1.98 / 2.58 | 2.00 / 2.87 | 0.38 / 0.71 | 1.68 / 3.78 |
| 128 | **0.73 / 0.94** | 14.76 / 22.07 | 4.02 / 4.85 | 4.28 / 5.80 | 0.74 / 1.44 | 3.45 / 6.21 |
| 256 | **1.47 / 1.90** | 29.65 / 44.12 | 8.61 / 12.24 | 8.71 / 10.87 | 1.44 / 2.96 | 7.01 / 10.93 |

Ranked on p99.

#### counter: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.02 / 0.04** | 0.12 / 0.40 | 0.07 / 0.12 | 0.09 / 0.13 | 0.04 / 0.06 | 0.09 / 0.13 |
| 8 | **0.06 / 0.10** | 0.78 / 1.96 | 0.25 / 0.47 | 0.26 / 0.55 | 0.08 / 0.12 | 0.21 / 0.48 |
| 32 | **0.18 / 0.27** | 3.49 / 7.22 | 0.99 / 1.44 | 1.01 / 1.54 | 0.20 / 0.35 | 0.79 / 2.48 |
| 64 | **0.35 / 0.50** | 7.32 / 11.10 | 2.00 / 2.74 | 2.03 / 2.94 | 0.38 / 0.70 | 1.67 / 3.87 |
| 128 | **0.72 / 0.94** | 14.78 / 20.95 | 4.01 / 4.83 | 4.38 / 6.06 | 0.73 / 1.42 | 3.43 / 6.22 |
| 256 | **1.47 / 1.71** | 29.69 / 43.53 | 8.56 / 12.28 | 8.85 / 11.96 | 1.46 / 3.09 | 6.93 / 10.68 |

Ranked on p99.

#### catalog: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.14 / 0.17** | 1.12 / 1.93 | 0.16 / 0.39 | 0.54 / 0.96 | 0.24 / 0.27 | 0.30 / 0.35 |
| 8 | **0.21 / 0.29** | 8.67 / 11.93 | 1.04 / 2.53 | 4.00 / 6.73 | 0.34 / 0.47 | 0.54 / 1.04 |
| 32 | **0.56 / 1.32** | 35.69 / 44.74 | 4.92 / 8.10 | 14.54 / 35.95 | 0.87 / 1.99 | 2.41 / 4.55 |
| 64 | **1.15 / 2.41** | 71.75 / 92.39 | 9.84 / 16.43 | 31.14 / 78.48 | 1.78 / 3.76 | 4.92 / 7.50 |
| 128 | **2.33 / 5.01** | 141.70 / 308.39 | 19.94 / 33.75 | 60.43 / 129.43 | 3.59 / 8.47 | 9.88 / 13.70 |
| 256 | **4.41 / 10.63** | 215.52 / 4341.42 | 39.61 / 62.80 | 76.29 / 887.64 | 6.98 / 20.91 | 19.76 / 25.56 |

Ranked on p99.

#### product: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.07 / 0.10** | 0.69 / 1.47 | 0.08 / 0.38 | 0.29 / 1.01 | 0.08 / 0.12 | 0.11 / 0.16 |
| 8 | **0.11 / 0.17** | 5.09 / 7.68 | 0.47 / 1.38 | 1.85 / 4.36 | 0.14 / 0.21 | 0.25 / 0.53 |
| 32 | **0.25 / 0.59** | 20.57 / 27.78 | 2.15 / 4.00 | 8.36 / 17.49 | 0.33 / 0.62 | 0.91 / 2.53 |
| 64 | **0.52 / 1.08** | 42.86 / 54.46 | 4.64 / 8.21 | 14.93 / 38.24 | 0.63 / 1.27 | 1.93 / 4.29 |
| 128 | **1.03 / 2.23** | 83.64 / 106.71 | 9.27 / 16.56 | 28.29 / 78.56 | 1.19 / 3.06 | 3.96 / 6.89 |
| 256 | **2.00 / 4.63** | 156.84 / 996.46 | 19.04 / 33.90 | 51.06 / 131.32 | 2.31 / 7.12 | 7.95 / 11.64 |

Ranked on p99.

#### search: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.08 / 0.10** | 0.72 / 1.40 | 0.09 / 0.40 | 0.34 / 0.95 | 0.11 / 0.15 | 0.14 / 0.23 |
| 8 | **0.13 / 0.19** | 5.42 / 7.95 | 0.58 / 1.75 | 2.17 / 4.92 | 0.17 / 0.25 | 0.30 / 0.57 |
| 32 | **0.30 / 0.77** | 21.26 / 28.38 | 2.80 / 5.10 | 9.69 / 20.29 | 0.39 / 1.00 | 1.12 / 2.96 |
| 64 | **0.62 / 1.37** | 44.45 / 56.45 | 5.59 / 9.62 | 17.26 / 45.79 | 0.77 / 1.72 | 2.32 / 4.60 |
| 128 | **1.22 / 2.61** | 86.82 / 110.80 | 11.64 / 20.23 | 33.17 / 89.04 | 1.53 / 3.71 | 4.68 / 7.44 |
| 256 | **2.44 / 5.77** | 166.36 / 1203.57 | 23.07 / 40.60 | 57.74 / 160.40 | 2.88 / 8.46 | 9.67 / 15.27 |

Ranked on p99.

#### missing: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **0.05 / 0.07** | 0.81 / 1.55 | 0.07 / 0.38 | 0.46 / 1.33 | 0.06 / 0.08 | 0.09 / 0.18 |
| 8 | **0.10 / 0.14** | 6.09 / 9.24 | 0.39 / 1.24 | 3.06 / 6.39 | 0.12 / 0.18 | 0.23 / 0.51 |
| 32 | **0.23 / 0.44** | 24.25 / 33.48 | 1.81 / 3.63 | 12.90 / 37.58 | 0.30 / 0.48 | 0.82 / 2.51 |
| 64 | 0.44 / 0.90 | 50.29 / 64.34 | 3.96 / 7.47 | 25.93 / 38.96 | **0.58 / 0.89** | 1.77 / 3.97 |
| 128 | 0.85 / 1.79 | 98.60 / 128.41 | 7.92 / 14.39 | 52.24 / 77.15 | **1.12 / 1.75** | 3.60 / 6.38 |
| 256 | 1.64 / 3.82 | 173.43 / 1745.76 | 15.94 / 29.21 | 95.81 / 143.04 | **2.17 / 3.45** | 7.28 / 11.09 |

Ranked on p99.

#### api: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.04 / 0.06 | 0.14 / 0.50 | 0.05 / 0.12 | 0.04 / 0.07 | **0.03 / 0.05** | 0.05 / 0.09 |
| 8 | 0.07 / 0.12 | 1.07 / 2.28 | 0.25 / 0.90 | 0.17 / 0.57 | **0.06 / 0.10** | 0.10 / 0.15 |
| 32 | 0.19 / 0.35 | 4.65 / 8.52 | 1.15 / 2.83 | 0.70 / 1.70 | **0.18 / 0.30** | 0.21 / 0.40 |
| 64 | 0.37 / 0.73 | 9.25 / 15.13 | 2.33 / 4.69 | 1.48 / 3.21 | **0.37 / 0.55** | 0.38 / 0.73 |
| 128 | 0.74 / 1.53 | 18.21 / 29.90 | 5.00 / 8.61 | 3.10 / 5.44 | **0.74 / 1.04** | 0.72 / 1.39 |
| 256 | 1.44 / 3.19 | 36.48 / 60.69 | 10.33 / 17.53 | 6.62 / 8.95 | **1.50 / 1.92** | 1.44 / 2.83 |

Ranked on p99.

#### primes: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 10.98 / 12.64<br>native 0.12 / 0.16 | 0.94 / 1.75 | 0.39 / 1.22 | 0.60 / 1.71 | **0.12 / 0.15** | 0.16 / 0.24 |
| 8 | 12.65 / 21.04<br>**native 0.18 / 0.26** | 7.46 / 9.17 | 2.59 / 6.45 | 4.40 / 7.51 | 0.19 / 0.28 | 0.26 / 0.46 |
| 32 | 37.93 / 67.81<br>native 0.42 / 1.04 | 30.39 / 37.67 | 11.45 / 22.47 | 19.44 / 38.25 | **0.42 / 0.99** | 0.92 / 2.38 |
| 64 | 73.84 / 138.98<br>native 0.88 / 1.92 | 61.03 / 78.44 | 23.34 / 34.44 | 32.99 / 80.82 | **0.83 / 1.81** | 1.92 / 3.73 |
| 128 | 144.95 / 285.68<br>**native 1.73 / 3.72** | 120.92 / 145.54 | 46.00 / 51.79 | 57.84 / 141.25 | 1.63 / 3.94 | 4.02 / 7.31 |
| 256 | 282.65 / 641.38<br>**native 3.38 / 8.01** | 200.52 / 2997.37 | 60.18 / 94.45 | 80.47 / 1394.83 | 3.16 / 9.31 | 8.05 / 12.20 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2.12 / 2.29<br>native 0.09 / 0.12 | 0.69 / 1.32 | 0.11 / 0.40 | 0.30 / 1.05 | **0.09 / 0.12** | 0.13 / 0.18 |
| 8 | 2.30 / 3.33<br>**native 0.14 / 0.20** | 5.25 / 7.26 | 0.66 / 1.75 | 1.89 / 4.38 | 0.15 / 0.23 | 0.24 / 0.45 |
| 32 | 6.29 / 12.39<br>native 0.32 / 0.91 | 20.54 / 27.20 | 3.03 / 5.15 | 9.40 / 17.15 | **0.35 / 0.67** | 0.89 / 2.45 |
| 64 | 12.73 / 24.61<br>**native 0.64 / 1.33** | 42.40 / 52.33 | 6.33 / 10.36 | 15.14 / 38.91 | 0.67 / 1.50 | 1.87 / 3.90 |
| 128 | 25.95 / 54.45<br>**native 1.29 / 2.69** | 85.83 / 107.93 | 12.76 / 20.76 | 30.50 / 80.45 | 1.31 / 3.23 | 3.81 / 6.48 |
| 256 | 50.41 / 116.47<br>**native 2.50 / 5.80** | 164.25 / 1100.28 | 25.80 / 43.31 | 51.68 / 135.75 | 2.52 / 7.15 | 7.68 / 11.27 |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 7.74 / 8.41<br>native 1.94 / 2.17 | 2.24 / 2.92 | 1.72 / 2.03 | 1.85 / 2.34 | **1.83 / 1.95** | 1.93 / 2.10 |
| 8 | 8.74 / 12.82<br>native 2.04 / 3.08 | 17.68 / 21.82 | 11.62 / 24.24 | 14.43 / 19.57 | 1.92 / 3.30 | **2.05 / 2.33** |
| 32 | 24.44 / 45.58<br>native 5.71 / 12.00 | 71.75 / 87.14 | 52.66 / 104.34 | 60.27 / 102.13 | **5.19 / 11.24** | 5.35 / 16.71 |
| 64 | 49.74 / 93.13<br>native 12.21 / 25.23 | 144.31 / 177.69 | 106.72 / 134.62 | 95.14 / 221.21 | **10.59 / 22.11** | 11.05 / 27.89 |
| 128 | 99.40 / 201.72<br>native 24.65 / 52.41 | 284.09 / 2205.41 | 130.51 / 3863.27 | 143.21 / 3100.20 | 20.82 / 50.66 | **23.00 / 43.96** |
| 256 | 196.66 / 449.88<br>native 47.77 / 112.90 | 325.78 / 7806.53 | 131.54 / 9079.51 | 147.25 / 8827.78 | 40.56 / 119.00 | **47.85 / 81.60** |

Ranked on p99.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: p50 / p99 ms

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.82 / 0.99 | 5.75 / 7.70 | 0.92 / 1.31 | 0.83 / 1.38 | 0.67 / 0.80 | **0.73 / 0.79** |
| 8 | 0.91 / 1.30 | 45.20 / 54.84 | 6.16 / 13.22 | 6.24 / 10.46 | **0.76 / 0.99** | 0.89 / 1.09 |
| 32 | 2.55 / 5.24 | 187.56 / 262.28 | 27.44 / 52.29 | 24.47 / 54.16 | **2.20 / 4.97** | 2.62 / 7.63 |
| 64 | 5.47 / 11.22 | 349.33 / 1781.81 | 54.22 / 73.18 | 45.67 / 106.35 | **4.48 / 9.40** | 5.55 / 13.04 |
| 128 | 11.22 / 23.74 | 480.01 / 9146.62 | 91.98 / 127.31 | 79.87 / 199.87 | **8.73 / 19.73** | 11.46 / 22.20 |
| 256 | 22.58 / 55.41 | 504.64 / 10179.42 | 92.75 / 4904.46 | 90.50 / 4292.60 | 16.74 / 47.07 | **23.07 / 38.01** |

Ranked on p99.

## CPU and memory

What the server used while it was loaded and what it takes to start.

#### Starting and idling

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Spawn to first 200 | **16 ms** | 262 ms | 52 ms | 215 ms | 23 ms | 16 ms |
| Resident at first 200 | 20.0 MB | 95.0 MB | 60.3 MB | 80.7 MB | **5.7 MB** | 10.2 MB |
| Peak resident over the run | **47.0 MB** | 582.0 MB | 363.5 MB | 667.0 MB | 111.5 MB | 180.8 MB |

#### home: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 49% / 31.7 MB | 101% / 283.0 MB | 136% / 142.1 MB | 91% / 154.0 MB | **67% / 18.7 MB** | 95% / 22.1 MB |
| 8 | 259% / 31.8 MB | 108% / 283.0 MB | 260% / 142.9 MB | 199% / 154.1 MB | **430% / 18.7 MB** | 596% / 22.8 MB |
| 32 | 436% / 33.3 MB | 108% / 283.0 MB | 237% / 143.6 MB | 203% / 154.8 MB | **771% / 19.4 MB** | 558% / 24.2 MB |
| 64 | 464% / 31.9 MB | 108% / 283.2 MB | 222% / 144.2 MB | 206% / 156.9 MB | **816% / 20.6 MB** | 555% / 26.8 MB |
| 128 | 479% / 31.2 MB | 108% / 289.8 MB | 217% / 145.8 MB | 200% / 161.1 MB | **836% / 22.8 MB** | 552% / 32.0 MB |
| 256 | 496% / 34.4 MB | 108% / 305.1 MB | 228% / 179.3 MB | 198% / 162.3 MB | **863% / 32.0 MB** | 551% / 42.4 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### counter: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 49% / 34.6 MB | 101% / 306.5 MB | 111% / 179.9 MB | 91% / 163.2 MB | **65% / 32.3 MB** | 97% / 42.5 MB |
| 8 | 256% / 34.6 MB | 108% / 306.5 MB | 212% / 179.9 MB | 199% / 163.2 MB | **424% / 32.4 MB** | 587% / 42.5 MB |
| 32 | 433% / 34.6 MB | 108% / 306.5 MB | 219% / 180.0 MB | 204% / 163.3 MB | **755% / 32.4 MB** | 541% / 42.7 MB |
| 64 | 458% / 34.6 MB | 108% / 306.5 MB | 220% / 180.4 MB | 207% / 163.4 MB | **808% / 32.4 MB** | 536% / 42.9 MB |
| 128 | 477% / 34.6 MB | 108% / 306.6 MB | 218% / 180.4 MB | 199% / 163.6 MB | **838% / 32.5 MB** | 542% / 43.1 MB |
| 256 | **493% / 34.6 MB** | 108% / 307.3 MB | 228% / 222.0 MB | 197% / 164.3 MB | 833% / 41.6 MB | 543% / 43.7 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### catalog: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 90% / 43.6 MB | 112% / 504.9 MB | 102% / 266.7 MB | 107% / 225.4 MB | **101% / 43.2 MB** | 110% / 51.2 MB |
| 8 | 659% / 43.7 MB | 117% / 505.0 MB | 110% / 266.7 MB | 116% / 225.4 MB | **757% / 43.2 MB** | 834% / 51.8 MB |
| 32 | 1114% / 44.9 MB | 122% / 516.5 MB | 110% / 269.1 MB | 116% / 249.7 MB | **1160% / 43.8 MB** | 895% / 52.7 MB |
| 64 | 1098% / 45.0 MB | 123% / 526.8 MB | 111% / 278.2 MB | 117% / 274.7 MB | **1163% / 44.5 MB** | 897% / 53.0 MB |
| 128 | **1103% / 45.0 MB** | 125% / 548.9 MB | 112% / 295.7 MB | 118% / 274.8 MB | 1153% / 47.0 MB | 906% / 53.4 MB |
| 256 | 1098% / 67.4 MB | 126% / 502.3 MB | 113% / 309.3 MB | 118% / 276.8 MB | 1134% / 63.1 MB | **894% / 55.1 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### product: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 78% / 67.8 MB | 107% / 514.6 MB | 100% / 311.5 MB | 110% / 277.1 MB | 100% / 64.0 MB | **101% / 55.4 MB** |
| 8 | 542% / 67.8 MB | 117% / 514.7 MB | 116% / 311.6 MB | 130% / 277.1 MB | 630% / 64.0 MB | **648% / 55.5 MB** |
| 32 | 976% / 68.0 MB | 119% / 515.0 MB | 117% / 311.7 MB | 129% / 277.1 MB | 923% / 64.1 MB | **650% / 55.6 MB** |
| 64 | 991% / 68.0 MB | 121% / 520.8 MB | 117% / 312.1 MB | 129% / 277.2 MB | 943% / 64.2 MB | **634% / 53.6 MB** |
| 128 | 1015% / 68.0 MB | 124% / 524.6 MB | 119% / 321.1 MB | 132% / 766.9 MB | 945% / 64.2 MB | **641% / 54.1 MB** |
| 256 | 1046% / 68.9 MB | 127% / 528.0 MB | 120% / 334.0 MB | 131% / 767.0 MB | 936% / 66.4 MB | **648% / 54.9 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### search: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 82% / 69.1 MB | 105% / 537.6 MB | 102% / 335.2 MB | 108% / 767.0 MB | 100% / 66.8 MB | **108% / 55.1 MB** |
| 8 | 578% / 69.1 MB | 117% / 537.6 MB | 115% / 335.1 MB | 125% / 767.0 MB | 685% / 66.8 MB | **710% / 55.1 MB** |
| 32 | 1010% / 69.1 MB | 120% / 537.6 MB | 116% / 335.5 MB | 125% / 767.0 MB | 1054% / 67.0 MB | **719% / 55.2 MB** |
| 64 | 1016% / 69.1 MB | 121% / 539.4 MB | 115% / 336.3 MB | 126% / 767.0 MB | 1071% / 67.1 MB | **723% / 53.1 MB** |
| 128 | 1052% / 69.1 MB | 124% / 539.7 MB | 117% / 340.4 MB | 126% / 767.0 MB | 1056% / 67.2 MB | **733% / 53.2 MB** |
| 256 | 1047% / 69.5 MB | 127% / 543.0 MB | 118% / 352.2 MB | 126% / 767.0 MB | 1060% / 67.6 MB | **717% / 53.6 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### missing: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 73% / 69.5 MB | 109% / 543.9 MB | 100% / 353.0 MB | 117% / 486.1 MB | 104% / 67.7 MB | **99% / 51.5 MB** |
| 8 | 500% / 69.5 MB | 118% / 543.9 MB | 119% / 352.9 MB | 130% / 486.1 MB | 547% / 67.7 MB | **598% / 51.5 MB** |
| 32 | 891% / 69.5 MB | 122% / 544.3 MB | 119% / 353.0 MB | 126% / 497.2 MB | 776% / 67.7 MB | **563% / 51.6 MB** |
| 64 | 914% / 69.5 MB | 124% / 544.5 MB | 120% / 353.1 MB | 131% / 504.3 MB | 789% / 67.7 MB | **554% / 51.7 MB** |
| 128 | 958% / 69.5 MB | 126% / 545.3 MB | 121% / 353.4 MB | 133% / 635.8 MB | 799% / 67.8 MB | **561% / 52.0 MB** |
| 256 | 987% / 69.5 MB | 127% / 546.8 MB | 122% / 361.2 MB | 133% / 636.5 MB | 810% / 67.8 MB | **564% / 52.6 MB** |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### api: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | **71% / 33.7 MB** | 111% / 556.7 MB | 94% / 326.1 MB | 75% / 636.5 MB | 59% / 68.1 MB | 59% / 52.9 MB |
| 8 | **433% / 33.8 MB** | 113% / 556.7 MB | 120% / 309.9 MB | 105% / 636.5 MB | 334% / 68.1 MB | 417% / 52.9 MB |
| 32 | **753% / 30.2 MB** | 113% / 556.8 MB | 121% / 310.5 MB | 105% / 636.6 MB | 570% / 68.1 MB | 775% / 53.5 MB |
| 64 | **832% / 31.0 MB** | 113% / 557.0 MB | 121% / 311.0 MB | 106% / 636.6 MB | 610% / 68.1 MB | 763% / 53.6 MB |
| 128 | **867% / 32.6 MB** | 113% / 556.9 MB | 122% / 311.9 MB | 106% / 636.6 MB | 624% / 68.1 MB | 794% / 54.0 MB |
| 256 | **882% / 36.0 MB** | 113% / 556.7 MB | 123% / 314.6 MB | 107% / 636.7 MB | 643% / 68.3 MB | 793% / 96.9 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

#### primes: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 181.0 MB<br>**native 88% / 42.3 MB** | 103% / 560.6 MB | 93% / 336.3 MB | 105% / 646.2 MB | 102% / 70.2 MB | 97% / 98.0 MB |
| 8 | 716% / 178.9 MB<br>**native 623% / 40.1 MB** | 110% / 565.2 MB | 109% / 334.9 MB | 115% / 649.3 MB | 647% / 70.2 MB | 686% / 98.2 MB |
| 32 | 1243% / 181.4 MB<br>**native 1059% / 41.2 MB** | 112% / 562.6 MB | 112% / 330.7 MB | 116% / 652.0 MB | 1048% / 70.6 MB | 749% / 98.3 MB |
| 64 | 1277% / 181.5 MB<br>**native 1038% / 43.5 MB** | 115% / 568.6 MB | 111% / 337.0 MB | 117% / 648.6 MB | 1079% / 70.8 MB | 751% / 98.4 MB |
| 128 | 1300% / 181.5 MB<br>**native 1069% / 45.0 MB** | 116% / 564.2 MB | 106% / 340.9 MB | 116% / 642.6 MB | 1083% / 70.8 MB | 728% / 77.8 MB |
| 256 | 1293% / 181.6 MB<br>**native 1074% / 48.5 MB** | 118% / 553.3 MB | 106% / 334.4 MB | 116% / 646.5 MB | 1070% / 71.8 MB | 739% / 84.2 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### report: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 97% / 175.0 MB<br>**native 84% / 48.9 MB** | 106% / 551.1 MB | 101% / 316.8 MB | 110% / 637.5 MB | 100% / 72.0 MB | 101% / 84.3 MB |
| 8 | 720% / 147.5 MB<br>**native 583% / 46.6 MB** | 118% / 551.9 MB | 113% / 316.8 MB | 129% / 637.5 MB | 642% / 72.0 MB | 667% / 84.3 MB |
| 32 | 1291% / 126.8 MB<br>**native 1003% / 46.9 MB** | 121% / 551.9 MB | 113% / 317.0 MB | 129% / 637.5 MB | 976% / 72.0 MB | 683% / 84.4 MB |
| 64 | 1280% / 117.4 MB<br>**native 1064% / 47.3 MB** | 122% / 554.9 MB | 114% / 317.3 MB | 129% / 637.5 MB | 993% / 72.1 MB | 679% / 84.4 MB |
| 128 | 1266% / 117.4 MB<br>**native 1074% / 53.1 MB** | 125% / 557.7 MB | 115% / 320.2 MB | 130% / 637.5 MB | 1002% / 72.3 MB | 684% / 84.5 MB |
| 256 | 1289% / 117.4 MB<br>**native 1081% / 69.5 MB** | 128% / 566.1 MB | 116% / 331.6 MB | 131% / 637.6 MB | 1013% / 72.8 MB | 687% / 84.9 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### words: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 99% / 117.5 MB<br>**native 96% / 69.6 MB** | 105% / 566.2 MB | 98% / 332.2 MB | 102% / 637.6 MB | 96% / 72.9 MB | 96% / 85.4 MB |
| 8 | 710% / 117.5 MB<br>**native 719% / 69.6 MB** | 109% / 566.2 MB | 101% / 332.2 MB | 106% / 637.6 MB | 757% / 72.9 MB | 778% / 85.5 MB |
| 32 | 1252% / 117.5 MB<br>**native 1278% / 69.7 MB** | 107% / 566.5 MB | 104% / 332.9 MB | 106% / 637.7 MB | 1284% / 72.9 MB | 1234% / 93.4 MB |
| 64 | 1266% / 117.5 MB<br>**native 1191% / 69.7 MB** | 108% / 554.9 MB | 104% / 333.4 MB | 107% / 637.7 MB | 1292% / 72.9 MB | 1232% / 99.8 MB |
| 128 | 1252% / 117.5 MB<br>**native 1190% / 69.7 MB** | 110% / 533.5 MB | 104% / 334.0 MB | 105% / 637.7 MB | 1281% / 73.0 MB | 1244% / 144.8 MB |
| 256 | 1252% / 117.5 MB<br>**native 1224% / 69.7 MB** | 110% / 521.4 MB | 96% / 334.0 MB | 93% / 637.7 MB | 1246% / 73.3 MB | 1230% / 145.9 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

Under FSR's figure, marked native: the same page with its loader calling the Rust in `shared/compute.rs` through `#[native]`, the code Leptos and Dioxus run. The view and the render are FSR's either way. It is ranked with the other figures.

#### table: server CPU % and resident MB

| Clients | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 92% / 122.3 MB | 112% / 595.7 MB | 99% / 346.7 MB | 106% / 637.7 MB | **96% / 78.3 MB** | 104% / 164.2 MB |
| 8 | 707% / 122.3 MB | 118% / 596.6 MB | 108% / 346.8 MB | 114% / 637.7 MB | **777% / 78.4 MB** | 827% / 164.5 MB |
| 32 | 1201% / 122.8 MB | 120% / 608.7 MB | 110% / 391.7 MB | 116% / 637.7 MB | **1155% / 80.3 MB** | 1114% / 172.5 MB |
| 64 | 1152% / 122.8 MB | 121% / 619.1 MB | 109% / 403.5 MB | 118% / 637.8 MB | **1180% / 83.3 MB** | 1109% / 173.9 MB |
| 128 | 1160% / 122.8 MB | 122% / 638.6 MB | 107% / 403.5 MB | 119% / 637.9 MB | **1208% / 91.1 MB** | 1096% / 176.2 MB |
| 256 | 1120% / 122.8 MB | 113% / 675.6 MB | 108% / 405.0 MB | 118% / 638.1 MB | **1189% / 111.5 MB** | 1108% / 180.8 MB |

CPU is the server's user and system time over the cell's wall time, 100 per core. Ranked on resident memory at the end of the cell.

## Build

Wall time of a production build, from nothing compiled and after one page changed.

#### Production build

| Build | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| cold | 40.5 s | 2.8 s | **1.5 s** | 3.4 s | 48.5 s | 56.9 s |
| warm | **0.7 s** | 1.4 s | 1.3 s | 2.6 s | 14.0 s | 13.2 s |

## Size

What a deployment carries and what each page costs a browser.

#### Deploy

| Measure | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Deploy tree | 25.3 MiB | 44.5 MiB | **1.5 MiB** | 2.4 MiB | 13.3 MiB | 8.6 MiB |
| Files in the tree | 458 | 1,351 | 181 | 168 | **6** | **6** |
| Server binary as built | 21.7 MiB | - | - | - | 12.5 MiB | **7.3 MiB** |
| Server binary stripped | 16.7 MiB | - | - | - | 7.1 MiB | **4.7 MiB** |
| Runtime needed beside it | - | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | Node v23.9.0, 107.3 MiB | - | - |
| Total to deploy | 25.3 MiB | 151.8 MiB | 108.8 MiB | 109.7 MiB | 13.3 MiB | **8.6 MiB** |

The total is the deploy tree, which holds the server binary where there is one, plus the runtime that has to be installed beside it.

#### Document

| Page | SnapFire FSR | Next.js | SvelteKit | Nuxt | Leptos | Dioxus |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| home | 1.9 KiB (5.9 KiB raw) | 1.9 KiB (6.4 KiB raw) | 1.1 KiB (2.7 KiB raw) | 1.1 KiB (2.3 KiB raw) | 1.1 KiB (2.2 KiB raw) | **1.1 KiB (2.5 KiB raw)** |
| counter | 1.7 KiB (5.8 KiB raw) | 1.7 KiB (5.9 KiB raw) | 0.9 KiB (2.2 KiB raw) | 0.9 KiB (1.9 KiB raw) | 0.9 KiB (1.8 KiB raw) | **0.9 KiB (1.9 KiB raw)** |
| catalog | 2.5 KiB (14.4 KiB raw) | 3.7 KiB (34.2 KiB raw) | **2.4 KiB (15.0 KiB raw)** | 2.9 KiB (15.4 KiB raw) | 2.7 KiB (15.5 KiB raw) | 4.3 KiB (27.9 KiB raw) |
| product | 1.9 KiB (6.5 KiB raw) | 2.1 KiB (9.2 KiB raw) | **1.3 KiB (3.7 KiB raw)** | 1.3 KiB (3.5 KiB raw) | 1.6 KiB (4.0 KiB raw) | 1.7 KiB (4.9 KiB raw) |
| search | 2.0 KiB (7.7 KiB raw) | 2.4 KiB (12.9 KiB raw) | **1.5 KiB (5.4 KiB raw)** | 1.6 KiB (5.3 KiB raw) | 1.8 KiB (5.7 KiB raw) | 2.1 KiB (8.2 KiB raw) |
| primes | 1.7 KiB (5.7 KiB raw) | 1.8 KiB (6.8 KiB raw) | 1.0 KiB (2.4 KiB raw) | **1.0 KiB (2.2 KiB raw)** | 1.3 KiB (2.8 KiB raw) | 1.1 KiB (2.6 KiB raw) |
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
