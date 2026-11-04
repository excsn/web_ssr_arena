import type { Ctx } from "@snapfire/fsr/host";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export async function load({ query }: Ctx<"/search">) {
  const q = (query.q ?? "").trim().toLowerCase();
  const results = q === "" ? [] : PRODUCTS.filter((p) => p.name.toLowerCase().includes(q) || p.tags.includes(q)).map(summary);
  return { q, results };
}
