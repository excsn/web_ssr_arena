import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export const dynamic = "force-dynamic";

export function GET() {
  return Response.json({ count: PRODUCTS.length, products: PRODUCTS.map(summary) });
}
