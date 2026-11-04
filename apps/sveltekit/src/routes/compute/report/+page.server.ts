import { report } from "@src/compute";

export function load({ url }) {
  const orders = Math.min(Math.max(Number(url.searchParams.get("orders") ?? "5000") || 0, 1), 200000);
  return { result: report(orders) };
}
