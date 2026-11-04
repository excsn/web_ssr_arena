<script setup lang="ts">
import { table } from "@src/compute";
import TableView from "@src/views/Table.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `table-${route.query.rows}`,
  () => Promise.resolve(table(Math.min(Math.max(Number(route.query.rows ?? "500") || 0, 1), 5000))),
);
</script>

<template>
  <TableView v-if="data" :rows="data" />
</template>
