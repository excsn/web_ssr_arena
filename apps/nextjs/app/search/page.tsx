import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard";

export default async function Search({ searchParams }: { searchParams: Promise<{ q?: string }> }) {
  const q = ((await searchParams).q ?? "").trim().toLowerCase();
  const results = q === "" ? [] : PRODUCTS.filter((p) => p.name.toLowerCase().includes(q) || p.tags.includes(q)).map(summary);
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
