import Link from "next/link";
import { price } from "@src/format";

export interface Summary {
  slug: string;
  name: string;
  priceCents: number;
  rating: number;
  inStock: boolean;
}

export default function ProductCard({ product }: { product: Summary }) {
  return (
    <li className="card">
      <Link href={`/products/${product.slug}`}>
        <h3>{product.name}</h3>
      </Link>
      <p className="price">{price(product.priceCents)}</p>
      <p className="rating">{product.rating.toFixed(1)} / 5</p>
      {product.inStock ? <p className="stock">In stock</p> : <p className="stock out">Out of stock</p>}
    </li>
  );
}
