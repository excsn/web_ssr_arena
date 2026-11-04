import { words } from "@src/compute";

export function load({ url }) {
  const passes = Math.min(Math.max(Number(url.searchParams.get("passes") ?? "20") || 0, 1), 1000);
  return { result: words(passes) };
}
