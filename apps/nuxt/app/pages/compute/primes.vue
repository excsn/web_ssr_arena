<script setup lang="ts">
import { primes } from "@src/compute";
import PrimesView from "@src/views/Primes.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `primes-${route.query.n}`,
  () => Promise.resolve(primes(Math.min(Math.max(Number(route.query.n ?? "50000") || 0, 10), 1000000))),
);
</script>

<template>
  <PrimesView v-if="data" :result="data" />
</template>
