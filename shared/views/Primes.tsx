import type { PrimeSummary } from "@src/compute";

export default function PrimesView({ result }: { result: PrimeSummary }) {
  return (
    <section>
      <h1>Primes up to {result.n}</h1>
      <p className="digest">{`primes ${result.n} ${result.count} ${result.sum} ${result.last.join(",")}`}</p>
      <ul className="tags">
        {result.last.map((p) => (
          <li key={p}>{p}</li>
        ))}
      </ul>
    </section>
  );
}
