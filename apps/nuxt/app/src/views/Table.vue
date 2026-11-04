<script setup lang="ts">
import type { TableRow } from "@src/compute";
import { price, stockLabel } from "@src/format";

defineProps<{ rows: TableRow[] }>();
</script>

<template>
  <section>
    <h1>Inventory</h1>
    <p class="digest">{{ `table ${rows.length} ${rows[rows.length - 1]?.slug ?? "-"}` }}</p>
    <table>
      <tbody>
        <tr v-for="r in rows" :key="r.at" class="row">
          <td>{{ r.at }}</td>
          <td>{{ r.name }}</td>
          <td>{{ price(r.priceCents) }}</td>
          <td>{{ r.rating.toFixed(1) }}</td>
          <td>{{ stockLabel(r.stock) }}</td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
