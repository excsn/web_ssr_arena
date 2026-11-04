import { json } from "@sveltejs/kit";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export function GET() {
  return json({ count: PRODUCTS.length, products: PRODUCTS.map(summary) });
}
