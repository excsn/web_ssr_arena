<script setup lang="ts">
import type { Report } from "@src/compute";
import { price } from "@src/format";

defineProps<{ result: Report }>();
</script>

<template>
  <section>
    <h1>Sales report</h1>
    <p class="digest">{{ `report ${result.orders} ${result.totalCents} ${result.rows.map((r) => `${r.slug}:${r.units}`).join(",")}` }}</p>
    <table>
      <thead>
        <tr>
          <th>Product</th>
          <th>Units</th>
          <th>Revenue</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="r in result.rows" :key="r.slug">
          <td>{{ r.name }}</td>
          <td>{{ r.units }}</td>
          <td>{{ price(r.revenueCents) }}</td>
        </tr>
      </tbody>
    </table>
    <p>Total {{ price(result.totalCents) }}</p>
  </section>
</template>
