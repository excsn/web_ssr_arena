import Link from "next/link";
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard";

export const dynamic = "force-dynamic";

export default async function Products({ searchParams }: { searchParams: Promise<{ sort?: string }> }) {
  const sort = (await searchParams).sort === "price" ? "price" : "rating";
  const products = PRODUCTS.map(summary);
  if (sort === "price") products.sort((a, b) => a.priceCents - b.priceCents);
  else products.sort((a, b) => b.rating - a.rating);
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
