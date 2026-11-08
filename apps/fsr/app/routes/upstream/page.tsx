import ProductCard from "@src/ui/ProductCard";
import type { Summary } from "@src/ui/ProductCard";

export default function Upstream({ count, products }: { count: number; products: Summary[] }) {
  return (
    <section>
      <h1>From the upstream</h1>
      <p>{count} products from /api/products</p>
      <ul className="grid">
        {products.map((p) => (
          <ProductCard key={p.slug} product={p} />
        ))}
      </ul>
    </section>
  );
}
