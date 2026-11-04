import { notFound } from "next/navigation";
import { PRODUCTS } from "@src/catalog";
import { price, stockLabel } from "@src/format";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard";

export const dynamic = "force-dynamic";

export default async function ProductPage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const product = PRODUCTS.find((p) => p.slug === slug);
  if (!product) notFound();
  const related = PRODUCTS.filter((p) => p.slug !== product.slug && p.tags[0] === product.tags[0]).slice(0, 4).map(summary);
  return (
    <article className="product">
      <h1>{product.name}</h1>
      <p className="price">{price(product.priceCents)}</p>
      <p className="rating">{product.rating.toFixed(1)} / 5</p>
      <p className="stock">{stockLabel(product.stock)}</p>
      <p>{product.blurb}</p>
      <ul className="tags">
        {product.tags.map((t) => (
          <li key={t}>{t}</li>
        ))}
      </ul>
      <h2>Related</h2>
      <ul className="grid">
        {related.map((p) => (
          <ProductCard key={p.slug} product={p} />
        ))}
      </ul>
    </article>
  );
}
