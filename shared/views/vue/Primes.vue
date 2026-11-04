<script setup lang="ts">
import type { PrimeSummary } from "@src/compute";

defineProps<{ result: PrimeSummary }>();
</script>

<template>
  <section>
    <h1>Primes up to {{ result.n }}</h1>
    <p class="digest">{{ `primes ${result.n} ${result.count} ${result.sum} ${result.last.join(",")}` }}</p>
    <ul class="tags">
      <li v-for="p in result.last" :key="p">{{ p }}</li>
    </ul>
  </section>
</template>
