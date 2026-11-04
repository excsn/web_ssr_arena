import type { Ctx } from "@snapfire/fsr/host";

export async function load({ query, native }: Ctx<"/compute/native/words">) {
  const passes = Math.min(Math.max(Number(query.passes ?? "20") || 0, 1), 1000);
  return { result: native.compute.words({ passes }) };
}
