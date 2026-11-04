import { fail } from "@snapfire/fsr/host";
import type { Ctx } from "@snapfire/fsr/host";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";

export async function load({ params }: Ctx<"/products/{slug}">) {
  const product = PRODUCTS.find((p) => p.slug === params.slug);
  if (!product) fail("not_found", "there is no product with that name");
  const related = PRODUCTS.filter((p) => p.slug !== product.slug && p.tags[0] === product.tags[0]).slice(0, 4).map(summary);
  return { product, related };
}
