import { table } from "@src/compute";
import TableView from "@src/views/Table";

export const dynamic = "force-dynamic";

export default async function Page({ searchParams }: { searchParams: Promise<{ rows?: string }> }) {
  const rows = Math.min(Math.max(Number((await searchParams).rows ?? "500") || 0, 1), 5000);
  return <TableView rows={table(rows)} />;
}
