import { primes } from "@src/compute";

export function load({ url }) {
  const n = Math.min(Math.max(Number(url.searchParams.get("n") ?? "50000") || 0, 10), 1000000);
  return { result: primes(n) };
}
