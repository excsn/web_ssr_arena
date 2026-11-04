import type { Ctx } from "@snapfire/fsr/host";
import { primes } from "@src/compute";

export async function load({ query }: Ctx<"/compute/primes">) {
  const n = Math.min(Math.max(Number(query.n ?? "50000") || 0, 10), 1000000);
  return { result: primes(n) };
}
