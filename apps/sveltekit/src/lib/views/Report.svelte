<script lang="ts">
  import type { Report } from "@src/compute";
  import { price } from "@src/format";

  let { result }: { result: Report } = $props();
</script>

<section>
  <h1>Sales report</h1>
  <p class="digest">{`report ${result.orders} ${result.totalCents} ${result.rows.map((r) => `${r.slug}:${r.units}`).join(",")}`}</p>
  <table>
    <thead>
      <tr>
        <th>Product</th>
        <th>Units</th>
        <th>Revenue</th>
      </tr>
    </thead>
    <tbody>
      {#each result.rows as r (r.slug)}
        <tr>
          <td>{r.name}</td>
          <td>{r.units}</td>
          <td>{price(r.revenueCents)}</td>
        </tr>
      {/each}
    </tbody>
  </table>
  <p>Total {price(result.totalCents)}</p>
</section>
