<script lang="ts">
  import type { TableRow } from "@src/compute";
  import { price, stockLabel } from "@src/format";

  let { rows }: { rows: TableRow[] } = $props();
</script>

<section>
  <h1>Inventory</h1>
  <p class="digest">{`table ${rows.length} ${rows[rows.length - 1]?.slug ?? "-"}`}</p>
  <table>
    <tbody>
      {#each rows as r (r.at)}
        <tr class="row">
          <td>{r.at}</td>
          <td>{r.name}</td>
          <td>{price(r.priceCents)}</td>
          <td>{r.rating.toFixed(1)}</td>
          <td>{stockLabel(r.stock)}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</section>
