import type { Product } from "@src/catalog";
import { price, stockLabel } from "@src/format";
import ProductCard from "@src/ui/ProductCard";
import type { Summary } from "@src/ui/ProductCard";

export default function ProductPage({ product, related }: { product: Product; related: Summary[] }) {
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
