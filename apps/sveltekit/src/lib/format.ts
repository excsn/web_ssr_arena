export function price(cents: number): string {
  return `$${(cents / 100).toFixed(2)}`;
}

export function stockLabel(stock: number): string {
  if (stock === 0) return "Out of stock";
  if (stock < 5) return `Only ${stock} left`;
  return "In stock";
}
