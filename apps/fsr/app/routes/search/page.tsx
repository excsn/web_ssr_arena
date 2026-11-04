import ProductCard from "@src/ui/ProductCard";
import type { Summary } from "@src/ui/ProductCard";

export default function Search({ q, results }: { q: string; results: Summary[] }) {
  return (
    <section>
      <h1>Search</h1>
      <form action="/search" method="get">
        <input type="search" name="q" defaultValue={q} />
        <button type="submit">Search</button>
      </form>
      <p>
        {results.length} results for &quot;{q}&quot;
      </p>
      <ul className="grid">
        {results.map((p) => (
          <ProductCard key={p.slug} product={p} />
        ))}
      </ul>
    </section>
  );
}
