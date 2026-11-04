import { error } from "@sveltejs/kit";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export function load({ params }) {
  const product = PRODUCTS.find((p) => p.slug === params.slug);
  if (!product) error(404);
  const related = PRODUCTS.filter((p) => p.slug !== product.slug && p.tags[0] === product.tags[0]).slice(0, 4).map(summary);
  return { product, related };
}
