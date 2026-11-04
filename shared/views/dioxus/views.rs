use dioxus::prelude::*;

use crate::compute::{PrimeSummary, Report, TableRow, Words, table_digest};
use crate::format::{price, stock_label};

pub fn primes_view(result: &PrimeSummary) -> Element {
  rsx! {
    section {
      h1 { "Primes up to {result.n}" }
      p { class: "digest", {result.digest()} }
      ul { class: "tags",
        for p in result.last.iter() {
          li { key: "{p}", "{p}" }
        }
      }
    }
  }
}

pub fn report_view(result: &Report) -> Element {
  rsx! {
    section {
      h1 { "Sales report" }
      p { class: "digest", {result.digest()} }
      table {
        thead {
          tr {
            th { "Product" }
            th { "Units" }
            th { "Revenue" }
          }
        }
        tbody {
          for r in result.rows.iter() {
            tr { key: "{r.slug}",
              td { "{r.name}" }
              td { "{r.units}" }
              td { {price(r.revenue_cents)} }
            }
          }
        }
      }
      p { {format!("Total {}", price(result.total_cents))} }
    }
  }
}

pub fn words_view(result: &Words) -> Element {
  rsx! {
    section {
      h1 { "Word counts" }
      p { class: "digest", {result.digest()} }
      ol {
        for w in result.top.iter() {
          li { key: "{w.word}", "{w.word} {w.count}" }
        }
      }
    }
  }
}

pub fn table_view(rows: &[TableRow]) -> Element {
  rsx! {
    section {
      h1 { "Inventory" }
      p { class: "digest", {table_digest(rows)} }
      table {
        tbody {
          for r in rows.iter() {
            tr { key: "{r.at}", class: "row",
              td { "{r.at}" }
              td { "{r.name}" }
              td { {price(r.price_cents)} }
              td { {format!("{:.1}", r.rating)} }
              td { {stock_label(r.stock)} }
            }
          }
        }
      }
    }
  }
}
