use serde::{Deserialize, Serialize};

use crate::catalog::PRODUCTS;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimeSummary {
  pub n: u64,
  pub count: u64,
  pub sum: u64,
  pub last: Vec<u64>,
}

impl PrimeSummary {
  pub fn digest(&self) -> String {
    format!("primes {} {} {} {}", self.n, self.count, self.sum, self.last.iter().map(u64::to_string).collect::<Vec<_>>().join(","))
  }
}

pub fn primes(n: u64) -> PrimeSummary {
  let n = n as usize;
  let mut composite = vec![false; n + 1];
  let mut count = 0;
  let mut sum = 0;
  let mut found = Vec::new();
  for i in 2..=n {
    if composite[i] {
      continue;
    }
    count += 1;
    sum = (sum + i as u64) % 1_000_000_007;
    found.push(i as u64);
    let mut j = i * i;
    while j <= n {
      composite[j] = true;
      j += i;
    }
  }
  let last = found[found.len().saturating_sub(10)..].to_vec();
  PrimeSummary { n: n as u64, count, sum, last }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportRow {
  pub slug: String,
  pub name: String,
  pub units: u64,
  pub revenue_cents: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
  pub orders: u64,
  pub total_cents: u64,
  pub rows: Vec<ReportRow>,
}

impl Report {
  pub fn digest(&self) -> String {
    format!("report {} {} {}", self.orders, self.total_cents, self.rows.iter().map(|r| format!("{}:{}", r.slug, r.units)).collect::<Vec<_>>().join(","))
  }
}

pub fn report(orders: u64) -> Report {
  let mut seed: u64 = 1234;
  let mut units = vec![0; PRODUCTS.len()];
  let mut revenue = vec![0; PRODUCTS.len()];
  let mut total_cents = 0;
  for _ in 0..orders {
    seed = (seed * 75 + 74) % 65537;
    let at = (seed % PRODUCTS.len() as u64) as usize;
    let quantity = 1 + (seed / 48) % 5;
    let cents = PRODUCTS[at].price_cents * quantity;
    units[at] += quantity;
    revenue[at] += cents;
    total_cents += cents;
  }
  let mut rows: Vec<ReportRow> = PRODUCTS.iter().enumerate().map(|(i, p)| ReportRow { slug: p.slug.to_owned(), name: p.name.to_owned(), units: units[i], revenue_cents: revenue[i] }).collect();
  rows.sort_by(|a, b| b.revenue_cents.cmp(&a.revenue_cents).then_with(|| a.slug.cmp(&b.slug)));
  rows.truncate(15);
  Report { orders, total_cents, rows }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WordCount {
  pub word: String,
  pub count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Words {
  pub total: u64,
  pub distinct: u64,
  pub top: Vec<WordCount>,
}

impl Words {
  pub fn digest(&self) -> String {
    format!("words {} {} {}", self.total, self.distinct, self.top.iter().map(|w| format!("{}:{}", w.word, w.count)).collect::<Vec<_>>().join(","))
  }
}

pub fn words(passes: u64) -> Words {
  let mut counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
  let mut total = 0;
  for _ in 0..passes {
    for p in PRODUCTS {
      let text = format!("{} {} {}", p.name, p.blurb, p.tags.join(" "));
      for raw in text.split(' ') {
        let word = raw.to_lowercase().replacen('.', "", 1).replacen(',', "", 1);
        if word.is_empty() {
          continue;
        }
        *counts.entry(word).or_insert(0) += 1;
        total += 1;
      }
    }
  }
  let mut top: Vec<WordCount> = counts.into_iter().map(|(word, count)| WordCount { word, count }).collect();
  top.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
  let distinct = top.len() as u64;
  top.truncate(20);
  Words { total, distinct, top }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
  pub at: u64,
  pub slug: String,
  pub name: String,
  pub price_cents: u64,
  pub rating: f64,
  pub stock: u64,
}

pub fn table_digest(rows: &[TableRow]) -> String {
  format!("table {} {}", rows.len(), rows.last().map_or("-", |r| r.slug.as_str()))
}

pub fn table(rows: u64) -> Vec<TableRow> {
  (0..rows)
    .map(|i| {
      let p = &PRODUCTS[(i % PRODUCTS.len() as u64) as usize];
      TableRow { at: i + 1, slug: p.slug.to_owned(), name: p.name.to_owned(), price_cents: p.price_cents, rating: p.rating, stock: (p.stock + i) % 40 }
    })
    .collect()
}
