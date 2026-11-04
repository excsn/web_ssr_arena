import type { Ctx } from "@snapfire/fsr/host";

export async function load({ query, native }: Ctx<"/compute/native/primes">) {
  const n = Math.min(Math.max(Number(query.n ?? "50000") || 0, 10), 1000000);
  return { result: native.compute.primes({ n }) };
}
