import type { PrimeSummary } from "@src/compute";
import PrimesView from "@src/views/Primes";

export default function Page({ result }: { result: PrimeSummary }) {
  return <PrimesView result={result} />;
}
