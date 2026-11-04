import type { Ctx } from "@snapfire/fsr/host";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export async function GET(_ctx: Ctx<"/api/products">) {
  return { count: PRODUCTS.length, products: PRODUCTS.map(summary) };
}
