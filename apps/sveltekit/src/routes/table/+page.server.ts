import { table } from "@src/compute";

export function load({ url }) {
  const rows = Math.min(Math.max(Number(url.searchParams.get("rows") ?? "500") || 0, 1), 5000);
  return { rows: table(rows) };
}
