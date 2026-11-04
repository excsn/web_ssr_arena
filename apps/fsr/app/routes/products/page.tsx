import { Link } from "@snapfire/fsr/components";
import ProductCard from "@src/ui/ProductCard";
import type { Summary } from "@src/ui/ProductCard";

export default function Products({ sort, products }: { sort: string; products: Summary[] }) {
  return (
    <section>
      <h1>All products</h1>
      <p>
        {products.length} products by {sort}. Sort by <Link href="/products?sort=rating">rating</Link> or <Link href="/products?sort=price">price</Link>.
      </p>
      <ul className="grid">
        {products.map((p) => (
          <ProductCard key={p.slug} product={p} />
        ))}
      </ul>
    </section>
  );
}
