<script setup lang="ts">
import { report } from "@src/compute";
import ReportView from "@src/views/Report.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `report-${route.query.orders}`,
  () => Promise.resolve(report(Math.min(Math.max(Number(route.query.orders ?? "5000") || 0, 1), 200000))),
);
</script>

<template>
  <ReportView v-if="data" :result="data" />
</template>
