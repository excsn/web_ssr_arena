import type { Ctx } from "@snapfire/fsr/host";

export async function load({ services }: Ctx<"/upstream">) {
  const catalog = await services.upstream.listProducts();
  return { count: catalog.count, products: catalog.products.slice(0, 12) };
}
