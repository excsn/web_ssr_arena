use leptos::prelude::*;

use crate::compute::{PrimeSummary, Report, TableRow, Words, table_digest};
use crate::format::{price, stock_label};

#[component]
pub fn PrimesView(result: PrimeSummary) -> impl IntoView {
  view! {
    <section>
      <h1>{format!("Primes up to {}", result.n)}</h1>
      <p class="digest">{result.digest()}</p>
      <ul class="tags">{result.last.into_iter().map(|p| view! { <li>{p}</li> }).collect_view()}</ul>
    </section>
  }
}

#[component]
pub fn ReportView(result: Report) -> impl IntoView {
  view! {
    <section>
      <h1>"Sales report"</h1>
      <p class="digest">{result.digest()}</p>
      <table>
        <thead>
          <tr>
            <th>"Product"</th>
            <th>"Units"</th>
            <th>"Revenue"</th>
          </tr>
        </thead>
        <tbody>
          {result
            .rows
            .into_iter()
            .map(|r| view! {
              <tr>
                <td>{r.name}</td>
                <td>{r.units}</td>
                <td>{price(r.revenue_cents)}</td>
              </tr>
            })
            .collect_view()}
        </tbody>
      </table>
      <p>{format!("Total {}", price(result.total_cents))}</p>
    </section>
  }
}

#[component]
pub fn WordsView(result: Words) -> impl IntoView {
  view! {
    <section>
      <h1>"Word counts"</h1>
      <p class="digest">{result.digest()}</p>
      <ol>{result.top.into_iter().map(|w| view! { <li>{format!("{} {}", w.word, w.count)}</li> }).collect_view()}</ol>
    </section>
  }
}

#[component]
pub fn TableView(rows: Vec<TableRow>) -> impl IntoView {
  let digest = table_digest(&rows);
  view! {
    <section>
      <h1>"Inventory"</h1>
      <p class="digest">{digest}</p>
      <table>
        <tbody>
          {rows
            .into_iter()
            .map(|r| view! {
              <tr class="row">
                <td>{r.at}</td>
                <td>{r.name}</td>
                <td>{price(r.price_cents)}</td>
                <td>{format!("{:.1}", r.rating)}</td>
                <td>{stock_label(r.stock)}</td>
              </tr>
            })
            .collect_view()}
        </tbody>
      </table>
    </section>
  }
}
