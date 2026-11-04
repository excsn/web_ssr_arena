import type { Ctx } from "@snapfire/fsr/host";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export async function load({ query }: Ctx<"/products">) {
  const sort = query.sort === "price" ? "price" : "rating";
  const products = PRODUCTS.map(summary);
  if (sort === "price") products.sort((a, b) => a.priceCents - b.priceCents);
  else products.sort((a, b) => b.rating - a.rating);
  return { sort, products };
}
