import type { TableRow } from "@src/compute";
import { price, stockLabel } from "@src/format";

export default function TableView({ rows }: { rows: TableRow[] }) {
  return (
    <section>
      <h1>Inventory</h1>
      <p className="digest">{`table ${rows.length} ${rows[rows.length - 1]?.slug ?? "-"}`}</p>
      <table>
        <tbody>
          {rows.map((r) => (
            <tr key={r.at} className="row">
              <td>{r.at}</td>
              <td>{r.name}</td>
              <td>{price(r.priceCents)}</td>
              <td>{r.rating.toFixed(1)}</td>
              <td>{stockLabel(r.stock)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
