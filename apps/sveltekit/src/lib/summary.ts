import type { Product } from "@src/catalog";

export function summary(p: Product) {
  return { slug: p.slug, name: p.name, priceCents: p.priceCents, rating: p.rating, inStock: p.stock > 0 };
}
