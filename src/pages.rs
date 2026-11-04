//! The requests every application answers and what each answer has to hold.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Render {
  /// Built once at build time by both frameworks.
  Static,
  /// Rendered for each request: it reads the query or a parameter.
  Live,
  /// A JSON route handler.
  Api,
}

impl Render {
  pub fn name(self) -> &'static str {
    match self {
      Render::Static => "static",
      Render::Live => "live",
      Render::Api => "api",
    }
  }
}

pub struct Page {
  pub id: &'static str,
  pub path: &'static str,
  pub render: Render,
  pub status: u16,
  /// Text the body must contain.
  pub holds: &'static [&'static str],
  /// How many product cards the body must contain.
  pub cards: usize,
  /// Whether the body writes a `<p class="digest">` of what it computed, which every app must write alike.
  pub digest: bool,
  pub blurb: &'static str,
  /// The same page with its loader computing in Rust, for an app that can call Rust from a loader. Measured as `<id>.native` and shown under the page's own figure.
  pub native: Option<&'static str>,
}

pub const PAGES: &[Page] = &[
  Page { id: "home", path: "/", render: Render::Static, status: 200, holds: &["Gear for long days out", "Repairs for life"], cards: 0, digest: false, blurb: "The landing page: a hero and three features, no data.", native: None },
  Page { id: "counter", path: "/counter", render: Render::Static, status: 200, holds: &["One component that runs in the browser"], cards: 0, digest: false, blurb: "A fixed page holding one component that hydrates in the browser.", native: None },
  Page { id: "catalog", path: "/products?sort=price", render: Render::Live, status: 200, holds: &["48 products by price", "Juniper Bottle"], cards: 48, digest: false, blurb: "All 48 products, sorted by the query.", native: None },
  Page { id: "product", path: "/products/cedar-lantern", render: Render::Live, status: 200, holds: &["Cedar Lantern", "Related"], cards: 4, digest: false, blurb: "One product by its slug, with four related products.", native: None },
  Page { id: "search", path: "/search?q=pack", render: Render::Live, status: 200, holds: &["12 results for"], cards: 12, digest: false, blurb: "Products whose name or tag matches the query.", native: None },
  Page { id: "missing", path: "/products/nope", render: Render::Live, status: 404, holds: &[], cards: 0, digest: false, blurb: "A slug with no product, answered 404.", native: None },
  Page { id: "api", path: "/api/products", render: Render::Api, status: 200, holds: &["\"count\":48"], cards: 0, digest: false, blurb: "The catalog as JSON.", native: None },
  Page { id: "primes", path: "/compute/primes?n=50000", render: Render::Live, status: 200, holds: &["primes 50000 5133 "], cards: 0, digest: true, blurb: "A sieve of Eratosthenes up to the query's n: tight numeric loops over an array.", native: Some("/compute/native/primes?n=50000") },
  Page { id: "report", path: "/compute/report?orders=5000", render: Render::Live, status: 200, holds: &["report 5000 "], cards: 0, digest: true, blurb: "Generated orders aggregated per product, sorted by revenue with a comparator and formatted.", native: Some("/compute/native/report?orders=5000") },
  Page { id: "words", path: "/compute/words?passes=20", render: Render::Live, status: 200, holds: &["words "], cards: 0, digest: true, blurb: "Word frequencies over a built-up text: splitting, lowercasing and counting in an object.", native: Some("/compute/native/words?passes=20") },
  Page { id: "table", path: "/table?rows=500", render: Render::Live, status: 200, holds: &["table 500 "], cards: 0, digest: true, blurb: "500 rows rendered from simple data: the cost is the render, not the loader.", native: None },
];

pub fn by_id(id: &str) -> Option<&'static Page> {
  PAGES.iter().find(|p| p.id == id)
}

/// The suffix a native measurement's page id carries.
pub const NATIVE: &str = ".native";

impl Page {
  /// The ids and paths `app` is measured on for this page: the page itself, then its native form when the app has one.
  pub fn variants(&self, native: bool) -> Vec<(String, &'static str)> {
    let mut out = vec![(self.id.to_owned(), self.path)];
    if let (true, Some(path)) = (native, self.native) {
      out.push((format!("{}{NATIVE}", self.id), path));
    }
    out
  }
}
