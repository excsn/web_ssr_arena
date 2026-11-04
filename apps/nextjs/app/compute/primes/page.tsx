import { primes } from "@src/compute";
import PrimesView from "@src/views/Primes";

export const dynamic = "force-dynamic";

export default async function Page({ searchParams }: { searchParams: Promise<{ n?: string }> }) {
  const n = Math.min(Math.max(Number((await searchParams).n ?? "50000") || 0, 10), 1000000);
  return <PrimesView result={primes(n)} />;
}
