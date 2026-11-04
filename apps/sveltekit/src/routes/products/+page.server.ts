import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export function load({ url }) {
  const sort = url.searchParams.get("sort") === "price" ? "price" : "rating";
  const products = PRODUCTS.map(summary);
  if (sort === "price") products.sort((a, b) => a.priceCents - b.priceCents);
  else products.sort((a, b) => b.rating - a.rating);
  return { sort, products };
}
