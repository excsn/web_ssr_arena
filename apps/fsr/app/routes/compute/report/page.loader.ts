import type { Ctx } from "@snapfire/fsr/host";
import { report } from "@src/compute";

export async function load({ query }: Ctx<"/compute/report">) {
  const orders = Math.min(Math.max(Number(query.orders ?? "5000") || 0, 1), 200000);
  return { result: report(orders) };
}
