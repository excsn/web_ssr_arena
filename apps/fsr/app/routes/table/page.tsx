import type { TableRow } from "@src/compute";
import TableView from "@src/views/Table";

export default function Page({ rows }: { rows: TableRow[] }) {
  return <TableView rows={rows} />;
}
