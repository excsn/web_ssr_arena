use snapfire_fsr_macros::{Record, native};

use crate::compute;

// Numbers cross as f64 so a body reads them as JavaScript numbers, the type `shared/compute.ts` answers with; an integer field would reach it as a bigint.

#[derive(Clone, Record)]
pub struct PrimesAnswer {
  pub n: f64,
  pub count: f64,
  pub sum: f64,
  pub last: Vec<f64>,
}

#[derive(Clone, Record)]
pub struct ReportAnswerRow {
  pub slug: String,
  pub name: String,
  pub units: f64,
  pub revenue_cents: f64,
}

#[derive(Clone, Record)]
pub struct ReportAnswer {
  pub orders: f64,
  pub total_cents: f64,
  pub rows: Vec<ReportAnswerRow>,
}

#[derive(Clone, Record)]
pub struct WordsAnswerCount {
  pub word: String,
  pub count: f64,
}

#[derive(Clone, Record)]
pub struct WordsAnswer {
  pub total: f64,
  pub distinct: f64,
  pub top: Vec<WordsAnswerCount>,
}

/// `shared/compute.rs`, the Rust Leptos and Dioxus run, reached from a loader as `native.compute`.
#[derive(Clone, Default)]
pub struct Compute;

#[native]
impl Compute {
  pub fn primes(&self, n: f64) -> PrimesAnswer {
    let r = compute::primes(n as u64);
    PrimesAnswer { n: r.n as f64, count: r.count as f64, sum: r.sum as f64, last: r.last.into_iter().map(|v| v as f64).collect() }
  }

  pub fn report(&self, orders: f64) -> ReportAnswer {
    let r = compute::report(orders as u64);
    ReportAnswer {
      orders: r.orders as f64,
      total_cents: r.total_cents as f64,
      rows: r.rows.into_iter().map(|row| ReportAnswerRow { slug: row.slug, name: row.name, units: row.units as f64, revenue_cents: row.revenue_cents as f64 }).collect(),
    }
  }

  pub fn words(&self, passes: f64) -> WordsAnswer {
    let r = compute::words(passes as u64);
    WordsAnswer { total: r.total as f64, distinct: r.distinct as f64, top: r.top.into_iter().map(|w| WordsAnswerCount { word: w.word, count: w.count as f64 }).collect() }
  }
}
