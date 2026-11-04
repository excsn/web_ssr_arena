import { PRODUCTS } from "@src/catalog";

export interface PrimeSummary {
  n: number;
  count: number;
  sum: number;
  last: number[];
}

export function primes(n: number): PrimeSummary {
  const composite: boolean[] = [];
  for (let i = 0; i <= n; i++) composite.push(false);
  let count = 0;
  let sum = 0;
  const found: number[] = [];
  for (let i = 2; i <= n; i++) {
    if (composite[i]) continue;
    count += 1;
    sum = (sum + i) % 1000000007;
    found.push(i);
    for (let j = i * i; j <= n; j += i) composite[j] = true;
  }
  return { n, count, sum, last: found.slice(Math.max(0, found.length - 10)) };
}

export interface ReportRow {
  slug: string;
  name: string;
  units: number;
  revenueCents: number;
}

export interface Report {
  orders: number;
  totalCents: number;
  rows: ReportRow[];
}

export function report(orders: number): Report {
  let seed = 1234;
  const units: number[] = [];
  const revenue: number[] = [];
  for (let i = 0; i < PRODUCTS.length; i++) {
    units.push(0);
    revenue.push(0);
  }
  let totalCents = 0;
  for (let i = 0; i < orders; i++) {
    seed = (seed * 75 + 74) % 65537;
    const at = seed % PRODUCTS.length;
    const quantity = 1 + (Math.floor(seed / 48) % 5);
    const cents = PRODUCTS[at].priceCents * quantity;
    units[at] += quantity;
    revenue[at] += cents;
    totalCents += cents;
  }
  const rows: ReportRow[] = PRODUCTS.map((p, i) => ({ slug: p.slug, name: p.name, units: units[i], revenueCents: revenue[i] }));
  rows.sort((a, b) => b.revenueCents - a.revenueCents || (a.slug < b.slug ? -1 : a.slug > b.slug ? 1 : 0));
  return { orders, totalCents, rows: rows.slice(0, 15) };
}

export interface WordCount {
  word: string;
  count: number;
}

export interface Words {
  total: number;
  distinct: number;
  top: WordCount[];
}

export function words(passes: number): Words {
  const counts: { [word: string]: number } = {};
  let total = 0;
  for (let pass = 0; pass < passes; pass++) {
    for (const p of PRODUCTS) {
      const text = `${p.name} ${p.blurb} ${p.tags.join(" ")}`;
      for (const raw of text.split(" ")) {
        const word = raw.toLowerCase().replace(".", "").replace(",", "");
        if (word.length === 0) continue;
        counts[word] = (counts[word] ?? 0) + 1;
        total += 1;
      }
    }
  }
  const top: WordCount[] = Object.keys(counts).map((word) => ({ word, count: counts[word] }));
  top.sort((a, b) => b.count - a.count || (a.word < b.word ? -1 : a.word > b.word ? 1 : 0));
  return { total, distinct: top.length, top: top.slice(0, 20) };
}

export interface TableRow {
  at: number;
  slug: string;
  name: string;
  priceCents: number;
  rating: number;
  stock: number;
}

export function table(rows: number): TableRow[] {
  const out: TableRow[] = [];
  for (let i = 0; i < rows; i++) {
    const p = PRODUCTS[i % PRODUCTS.length];
    out.push({ at: i + 1, slug: p.slug, name: p.name, priceCents: p.priceCents, rating: p.rating, stock: (p.stock + i) % 40 });
  }
  return out;
}
