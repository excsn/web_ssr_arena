import type { Ctx } from "@snapfire/fsr/host";
import { table } from "@src/compute";

export async function load({ query }: Ctx<"/table">) {
  const rows = Math.min(Math.max(Number(query.rows ?? "500") || 0, 1), 5000);
  return { rows: table(rows) };
}
