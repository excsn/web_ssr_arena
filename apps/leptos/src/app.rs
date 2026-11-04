use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::hooks::{use_params_map, use_query_map};
use leptos_router::path;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::catalog::{PRODUCTS, Product};
use crate::compute::{PrimeSummary, Report, TableRow, Words};
use crate::format::{price, stock_label};
use crate::views::{PrimesView, ReportView, TableView, WordsView};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <HydrationScripts options />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
  pub slug: String,
  pub name: String,
  pub price_cents: u64,
  pub rating: f64,
  pub in_stock: bool,
}

#[cfg(feature = "ssr")]
fn summary(p: &Product) -> Summary {
  Summary { slug: p.slug.to_owned(), name: p.name.to_owned(), price_cents: p.price_cents, rating: p.rating, in_stock: p.stock > 0 }
}

#[cfg(feature = "ssr")]
#[derive(Serialize)]
struct Catalog {
  count: usize,
  products: Vec<Summary>,
}

#[cfg(feature = "ssr")]
pub async fn api_products() -> axum::Json<impl Serialize> {
  axum::Json(Catalog { count: PRODUCTS.len(), products: PRODUCTS.iter().map(summary).collect() })
}

/// A query value as JavaScript's `Number(value ?? default) || 0` reads it, held between `lo` and `hi`.
#[cfg(feature = "ssr")]
fn bounded(value: Option<String>, default: f64, lo: f64, hi: f64) -> u64 {
  let n = value.map_or(default, |v| v.trim().parse::<f64>().unwrap_or(0.0));
  let n = if n.is_nan() { 0.0 } else { n };
  n.max(lo).min(hi) as u64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductInfo {
  pub name: String,
  pub price_cents: u64,
  pub rating: f64,
  pub stock: u64,
  pub blurb: String,
  pub tags: Vec<String>,
  pub related: Vec<Summary>,
}

#[server]
pub async fn list_products(sort: Option<String>) -> Result<(String, Vec<Summary>), ServerFnError> {
  let sort = if sort.as_deref() == Some("price") { "price" } else { "rating" };
  let mut products: Vec<Summary> = PRODUCTS.iter().map(summary).collect();
  if sort == "price" {
    products.sort_by(|a, b| a.price_cents.cmp(&b.price_cents));
  } else {
    products.sort_by(|a, b| b.rating.total_cmp(&a.rating));
  }
  Ok((sort.to_owned(), products))
}

#[server]
pub async fn get_product(slug: String) -> Result<Option<ProductInfo>, ServerFnError> {
  let Some(product) = PRODUCTS.iter().find(|p| p.slug == slug) else { return Ok(None) };
  let related = PRODUCTS.iter().filter(|p| p.slug != product.slug && p.tags[0] == product.tags[0]).take(4).map(summary).collect();
  Ok(Some(ProductInfo {
    name: product.name.to_owned(),
    price_cents: product.price_cents,
    rating: product.rating,
    stock: product.stock,
    blurb: product.blurb.to_owned(),
    tags: product.tags.iter().map(|t| (*t).to_owned()).collect(),
    related,
  }))
}

#[server]
pub async fn search_products(q: Option<String>) -> Result<(String, Vec<Summary>), ServerFnError> {
  let q = q.unwrap_or_default().trim().to_lowercase();
  let results = if q.is_empty() { Vec::new() } else { PRODUCTS.iter().filter(|p| p.name.to_lowercase().contains(&q) || p.tags.contains(&q.as_str())).map(summary).collect() };
  Ok((q, results))
}

#[server]
pub async fn primes_page(n: Option<String>) -> Result<PrimeSummary, ServerFnError> {
  Ok(crate::compute::primes(bounded(n, 50000.0, 10.0, 1_000_000.0)))
}

#[server]
pub async fn report_page(orders: Option<String>) -> Result<Report, ServerFnError> {
  Ok(crate::compute::report(bounded(orders, 5000.0, 1.0, 200_000.0)))
}

#[server]
pub async fn words_page(passes: Option<String>) -> Result<Words, ServerFnError> {
  Ok(crate::compute::words(bounded(passes, 20.0, 1.0, 1000.0)))
}

#[server]
pub async fn table_page(rows: Option<String>) -> Result<Vec<TableRow>, ServerFnError> {
  Ok(crate::compute::table(bounded(rows, 500.0, 1.0, 5000.0)))
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  view! {
    <Stylesheet id="leptos" href="/pkg/outfitters_leptos.css" />
    <Title text="Outfitters" />
    <Router>
      <div class="shell">
        <header class="bar">
          <A href="/" attr:class="brand">"Outfitters"</A>
          <nav>
            <A href="/products">"Products"</A>
            <A href="/search?q=pack">"Search"</A>
            <A href="/counter">"Counter"</A>
          </nav>
        </header>
        <main class="content">
          <Routes fallback=NotFound>
            <Route path=path!("/") view=Home />
            <Route path=path!("/counter") view=CounterPage />
            <Route path=path!("/products") view=Products />
            <Route path=path!("/products/:slug") view=ProductPage />
            <Route path=path!("/search") view=Search />
            <Route path=path!("/compute/primes") view=Primes />
            <Route path=path!("/compute/report") view=SalesReport />
            <Route path=path!("/compute/words") view=WordCounts />
            <Route path=path!("/table") view=Inventory />
          </Routes>
        </main>
        <footer class="foot">"Outfitters, a benchmark storefront."</footer>
      </div>
    </Router>
  }
}

const FEATURES: &[(&str, &str)] = &[
  ("Built for the trail", "Every pack, jacket, lantern and bottle is tested over a full season before it is listed."),
  ("Repairs for life", "Send anything back with a tear or a broken buckle and it comes home fixed."),
  ("Shipped in a day", "Orders placed before noon leave the warehouse the same afternoon."),
];

#[component]
fn Home() -> impl IntoView {
  view! {
    <section class="hero">
      <h1>"Gear for long days out"</h1>
      <p>"Forty-eight things we carry ourselves, from packs to lanterns."</p>
      <A href="/products" attr:class="cta">"Browse the catalog"</A>
    </section>
    <section class="features">
      {FEATURES
        .iter()
        .map(|(title, body)| view! {
          <article>
            <h2>{*title}</h2>
            <p>{*body}</p>
          </article>
        })
        .collect_view()}
    </section>
  }
}

#[component]
fn NotFound() -> impl IntoView {
  #[cfg(feature = "ssr")]
  if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
    response.set_status(http::StatusCode::NOT_FOUND);
  }
  view! {
    <section class="hero">
      <h1>"No page here"</h1>
      <A href="/">"Back to the start"</A>
    </section>
  }
}

#[component]
fn Counter(start: i64) -> impl IntoView {
  let (count, set_count) = signal(start);
  view! {
    <div class="counter">
      <button type="button" on:click=move |_| set_count.update(|c| *c -= 1)>"-"</button>
      <output>{count}</output>
      <button type="button" on:click=move |_| set_count.update(|c| *c += 1)>"+"</button>
    </div>
  }
}

#[component]
fn CounterPage() -> impl IntoView {
  view! {
    <section>
      <h1>"Counter"</h1>
      <p>"One component that runs in the browser."</p>
      <Counter start=3 />
    </section>
  }
}

#[component]
fn ProductCard(product: Summary) -> impl IntoView {
  view! {
    <li class="card">
      <A href=format!("/products/{}", product.slug)>
        <h3>{product.name}</h3>
      </A>
      <p class="price">{price(product.price_cents)}</p>
      <p class="rating">{format!("{:.1} / 5", product.rating)}</p>
      {if product.in_stock { view! { <p class="stock">"In stock"</p> }.into_any() } else { view! { <p class="stock out">"Out of stock"</p> }.into_any() }}
    </li>
  }
}

fn cards(products: Vec<Summary>) -> impl IntoView {
  view! { <ul class="grid">{products.into_iter().map(|p| view! { <ProductCard product=p /> }).collect_view()}</ul> }
}

#[component]
fn Products() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("sort"), list_products);
  view! {
    <Suspense>
      {move || Suspend::new(async move {
        data.await.map(|(sort, products)| view! {
          <section>
            <h1>"All products"</h1>
            <p>
              {format!("{} products by {}. Sort by ", products.len(), sort)}
              <A href="/products?sort=rating">"rating"</A>
              " or "
              <A href="/products?sort=price">"price"</A>
              "."
            </p>
            {cards(products)}
          </section>
        })
      })}
    </Suspense>
  }
}

#[component]
fn ProductPage() -> impl IntoView {
  let params = use_params_map();
  let data = Resource::new_blocking(move || params.read().get("slug").unwrap_or_default(), get_product);
  view! {
    <Suspense>
      {move || Suspend::new(async move {
        match data.await {
          Ok(Some(product)) => view! {
            <article class="product">
              <h1>{product.name}</h1>
              <p class="price">{price(product.price_cents)}</p>
              <p class="rating">{format!("{:.1} / 5", product.rating)}</p>
              <p class="stock">{stock_label(product.stock)}</p>
              <p>{product.blurb}</p>
              <ul class="tags">{product.tags.into_iter().map(|t| view! { <li>{t}</li> }).collect_view()}</ul>
              <h2>"Related"</h2>
              {cards(product.related)}
            </article>
          }
          .into_any(),
          _ => view! { <NotFound /> }.into_any(),
        }
      })}
    </Suspense>
  }
}

#[component]
fn Search() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("q"), search_products);
  view! {
    <Suspense>
      {move || Suspend::new(async move {
        data.await.map(|(q, results)| view! {
          <section>
            <h1>"Search"</h1>
            <form action="/search" method="get">
              <input type="search" name="q" value=q.clone() />
              <button type="submit">"Search"</button>
            </form>
            <p>{format!("{} results for \"{}\"", results.len(), q)}</p>
            {cards(results)}
          </section>
        })
      })}
    </Suspense>
  }
}

#[component]
fn Primes() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("n"), primes_page);
  view! { <Suspense>{move || Suspend::new(async move { data.await.map(|result| view! { <PrimesView result /> }) })}</Suspense> }
}

#[component]
fn SalesReport() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("orders"), report_page);
  view! { <Suspense>{move || Suspend::new(async move { data.await.map(|result| view! { <ReportView result /> }) })}</Suspense> }
}

#[component]
fn WordCounts() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("passes"), words_page);
  view! { <Suspense>{move || Suspend::new(async move { data.await.map(|result| view! { <WordsView result /> }) })}</Suspense> }
}

#[component]
fn Inventory() -> impl IntoView {
  let query = use_query_map();
  let data = Resource::new_blocking(move || query.read().get("rows"), table_page);
  view! { <Suspense>{move || Suspend::new(async move { data.await.map(|rows| view! { <TableView rows /> }) })}</Suspense> }
}
