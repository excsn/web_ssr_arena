#[cfg_attr(not(feature = "server"), allow(dead_code))]
mod catalog;
#[cfg_attr(not(feature = "server"), allow(dead_code))]
mod compute;
mod format;
mod views;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::catalog::{PRODUCTS, Product};
use crate::compute::{PrimeSummary, Report, TableRow, Words};
use crate::format::{price, stock_label};

fn main() {
  dioxus::launch(App);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
  pub slug: String,
  pub name: String,
  pub price_cents: u64,
  pub rating: f64,
  pub in_stock: bool,
}

#[cfg(feature = "server")]
fn summary(p: &Product) -> Summary {
  Summary { slug: p.slug.to_owned(), name: p.name.to_owned(), price_cents: p.price_cents, rating: p.rating, in_stock: p.stock > 0 }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Catalog {
  count: usize,
  products: Vec<Summary>,
}

#[get("/api/products")]
async fn api_products() -> Result<Catalog, ServerFnError> {
  Ok(Catalog { count: PRODUCTS.len(), products: PRODUCTS.iter().map(summary).collect() })
}

/// A query value as JavaScript's `Number(value ?? default) || 0` reads it, held between `lo` and `hi`. The router gives a missing value as empty.
#[cfg(feature = "server")]
fn bounded(value: &str, default: f64, lo: f64, hi: f64) -> u64 {
  let n = if value.is_empty() { default } else { value.trim().parse::<f64>().unwrap_or(0.0) };
  let n = if n.is_nan() { 0.0 } else { n };
  n.max(lo).min(hi) as u64
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
async fn list_products(sort: String) -> Result<(String, Vec<Summary>), ServerFnError> {
  let sort = if sort == "price" { "price" } else { "rating" };
  let mut products: Vec<Summary> = PRODUCTS.iter().map(summary).collect();
  if sort == "price" {
    products.sort_by(|a, b| a.price_cents.cmp(&b.price_cents));
  } else {
    products.sort_by(|a, b| b.rating.total_cmp(&a.rating));
  }
  Ok((sort.to_owned(), products))
}

#[server]
async fn get_product(slug: String) -> Result<Option<ProductInfo>, ServerFnError> {
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
async fn search_products(q: String) -> Result<(String, Vec<Summary>), ServerFnError> {
  let q = q.trim().to_lowercase();
  let results = if q.is_empty() { Vec::new() } else { PRODUCTS.iter().filter(|p| p.name.to_lowercase().contains(&q) || p.tags.contains(&q.as_str())).map(summary).collect() };
  Ok((q, results))
}

#[server]
async fn primes_page(n: String) -> Result<PrimeSummary, ServerFnError> {
  Ok(crate::compute::primes(bounded(&n, 50000.0, 10.0, 1_000_000.0)))
}

#[server]
async fn report_page(orders: String) -> Result<Report, ServerFnError> {
  Ok(crate::compute::report(bounded(&orders, 5000.0, 1.0, 200_000.0)))
}

#[server]
async fn words_page(passes: String) -> Result<Words, ServerFnError> {
  Ok(crate::compute::words(bounded(&passes, 20.0, 1.0, 1000.0)))
}

#[server]
async fn table_page(rows: String) -> Result<Vec<TableRow>, ServerFnError> {
  Ok(crate::compute::table(bounded(&rows, 500.0, 1.0, 5000.0)))
}

#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
  #[layout(Shell)]
  #[route("/")]
  Home {},
  #[route("/counter")]
  CounterPage {},
  #[route("/products?:sort")]
  Products { sort: String },
  #[route("/products/:slug")]
  ProductPage { slug: String },
  #[route("/search?:q")]
  Search { q: String },
  #[route("/compute/primes?:n")]
  Primes { n: String },
  #[route("/compute/report?:orders")]
  SalesReport { orders: String },
  #[route("/compute/words?:passes")]
  WordCounts { passes: String },
  #[route("/table?:rows")]
  Inventory { rows: String },
  #[route("/:..segments")]
  NotFound { segments: Vec<String> },
}

const STYLES: Asset = asset!("/assets/app.css");

#[component]
fn App() -> Element {
  rsx! {
    document::Title { "Outfitters" }
    document::Stylesheet { href: STYLES }
    Router::<Route> {}
  }
}

#[component]
fn Shell() -> Element {
  rsx! {
    div { class: "shell",
      header { class: "bar",
        Link { to: "/", class: "brand", "Outfitters" }
        nav {
          Link { to: "/products", "Products" }
          Link { to: "/search?q=pack", "Search" }
          Link { to: "/counter", "Counter" }
        }
      }
      main { class: "content", Outlet::<Route> {} }
      footer { class: "foot", "Outfitters, a benchmark storefront." }
    }
  }
}

const FEATURES: &[(&str, &str)] = &[
  ("Built for the trail", "Every pack, jacket, lantern and bottle is tested over a full season before it is listed."),
  ("Repairs for life", "Send anything back with a tear or a broken buckle and it comes home fixed."),
  ("Shipped in a day", "Orders placed before noon leave the warehouse the same afternoon."),
];

#[component]
fn Home() -> Element {
  rsx! {
    section { class: "hero",
      h1 { "Gear for long days out" }
      p { "Forty-eight things we carry ourselves, from packs to lanterns." }
      Link { to: "/products", class: "cta", "Browse the catalog" }
    }
    section { class: "features",
      for (title, body) in FEATURES.iter() {
        article { key: "{title}",
          h2 { "{title}" }
          p { "{body}" }
        }
      }
    }
  }
}

fn missing() -> Element {
  #[cfg(feature = "server")]
  dioxus::fullstack::FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);
  rsx! {
    section { class: "hero",
      h1 { "No page here" }
      Link { to: "/", "Back to the start" }
    }
  }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
  missing()
}

#[component]
fn Counter(start: i64) -> Element {
  let mut count = use_signal(|| start);
  rsx! {
    div { class: "counter",
      button { r#type: "button", onclick: move |_| count -= 1, "-" }
      output { "{count}" }
      button { r#type: "button", onclick: move |_| count += 1, "+" }
    }
  }
}

#[component]
fn CounterPage() -> Element {
  rsx! {
    section {
      h1 { "Counter" }
      p { "One component that runs in the browser." }
      Counter { start: 3 }
    }
  }
}

fn product_card(product: &Summary) -> Element {
  rsx! {
    li { key: "{product.slug}", class: "card",
      Link { to: "/products/{product.slug}",
        h3 { "{product.name}" }
      }
      p { class: "price", {price(product.price_cents)} }
      p { class: "rating", {format!("{:.1} / 5", product.rating)} }
      if product.in_stock {
        p { class: "stock", "In stock" }
      } else {
        p { class: "stock out", "Out of stock" }
      }
    }
  }
}

fn cards(products: &[Summary]) -> Element {
  rsx! {
    ul { class: "grid",
      for p in products.iter() {
        {product_card(p)}
      }
    }
  }
}

#[component]
fn Products(sort: String) -> Element {
  let data = use_server_future(move || list_products(sort.clone()))?;
  let Some(Ok((sort, products))) = data() else { return rsx! {} };
  rsx! {
    section {
      h1 { "All products" }
      p {
        {format!("{} products by {}. Sort by ", products.len(), sort)}
        Link { to: "/products?sort=rating", "rating" }
        " or "
        Link { to: "/products?sort=price", "price" }
        "."
      }
      {cards(&products)}
    }
  }
}

#[component]
fn ProductPage(slug: String) -> Element {
  let data = use_server_future(move || get_product(slug.clone()))?;
  let Some(Ok(Some(product))) = data() else { return missing() };
  rsx! {
    article { class: "product",
      h1 { "{product.name}" }
      p { class: "price", {price(product.price_cents)} }
      p { class: "rating", {format!("{:.1} / 5", product.rating)} }
      p { class: "stock", {stock_label(product.stock)} }
      p { "{product.blurb}" }
      ul { class: "tags",
        for t in product.tags.iter() {
          li { key: "{t}", "{t}" }
        }
      }
      h2 { "Related" }
      {cards(&product.related)}
    }
  }
}

#[component]
fn Search(q: String) -> Element {
  let data = use_server_future(move || search_products(q.clone()))?;
  let Some(Ok((q, results))) = data() else { return rsx! {} };
  rsx! {
    section {
      h1 { "Search" }
      form { action: "/search", method: "get",
        input { r#type: "search", name: "q", value: "{q}" }
        button { r#type: "submit", "Search" }
      }
      p { {format!("{} results for \"{}\"", results.len(), q)} }
      {cards(&results)}
    }
  }
}

#[component]
fn Primes(n: String) -> Element {
  let data = use_server_future(move || primes_page(n.clone()))?;
  let Some(Ok(result)) = data() else { return rsx! {} };
  views::primes_view(&result)
}

#[component]
fn SalesReport(orders: String) -> Element {
  let data = use_server_future(move || report_page(orders.clone()))?;
  let Some(Ok(result)) = data() else { return rsx! {} };
  views::report_view(&result)
}

#[component]
fn WordCounts(passes: String) -> Element {
  let data = use_server_future(move || words_page(passes.clone()))?;
  let Some(Ok(result)) = data() else { return rsx! {} };
  views::words_view(&result)
}

#[component]
fn Inventory(rows: String) -> Element {
  let data = use_server_future(move || table_page(rows.clone()))?;
  let Some(Ok(rows)) = data() else { return rsx! {} };
  views::table_view(&rows)
}
