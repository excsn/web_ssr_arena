<script setup lang="ts">
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `search-${route.query.q}`,
  () => {
    const q = String(route.query.q ?? "").trim().toLowerCase();
    const results = q === "" ? [] : PRODUCTS.filter((p) => p.name.toLowerCase().includes(q) || p.tags.includes(q)).map(summary);
    return Promise.resolve({ q, results });
  },
);
</script>

<template>
  <section v-if="data">
    <h1>Search</h1>
    <form action="/search" method="get">
      <input type="search" name="q" :value="data.q" />
      <button type="submit">Search</button>
    </form>
    <p>{{ data.results.length }} results for &quot;{{ data.q }}&quot;</p>
    <ul class="grid">
      <ProductCard v-for="p in data.results" :key="p.slug" :product="p" />
    </ul>
  </section>
</template>
