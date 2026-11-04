pub fn price(cents: u64) -> String {
  format!("${:.2}", cents as f64 / 100.0)
}

pub fn stock_label(stock: u64) -> String {
  match stock {
    0 => "Out of stock".to_owned(),
    1..5 => format!("Only {stock} left"),
    _ => "In stock".to_owned(),
  }
}
