import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export function load({ url }) {
  const q = (url.searchParams.get("q") ?? "").trim().toLowerCase();
  const results = q === "" ? [] : PRODUCTS.filter((p) => p.name.toLowerCase().includes(q) || p.tags.includes(q)).map(summary);
  return { q, results };
}
