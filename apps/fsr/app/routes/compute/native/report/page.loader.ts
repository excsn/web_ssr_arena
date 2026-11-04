import type { Ctx } from "@snapfire/fsr/host";

export async function load({ query, native }: Ctx<"/compute/native/report">) {
  const orders = Math.min(Math.max(Number(query.orders ?? "5000") || 0, 1), 200000);
  return { result: native.compute.report({ orders }) };
}
