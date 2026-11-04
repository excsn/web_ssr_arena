import type { Ctx } from "@snapfire/fsr/host";
import { words } from "@src/compute";

export async function load({ query }: Ctx<"/compute/words">) {
  const passes = Math.min(Math.max(Number(query.passes ?? "20") || 0, 1), 1000);
  return { result: words(passes) };
}
