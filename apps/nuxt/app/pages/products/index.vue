<script setup lang="ts">
import { PRODUCTS } from "@src/catalog";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `products-${route.query.sort}`,
  () => {
    const sort = route.query.sort === "price" ? "price" : "rating";
    const products = PRODUCTS.map(summary);
    if (sort === "price") products.sort((a, b) => a.priceCents - b.priceCents);
    else products.sort((a, b) => b.rating - a.rating);
    return Promise.resolve({ sort, products });
  },
);
</script>

<template>
  <section v-if="data">
    <h1>All products</h1>
    <p>
      {{ data.products.length }} products by {{ data.sort }}. Sort by <NuxtLink to="/products?sort=rating">rating</NuxtLink> or <NuxtLink to="/products?sort=price">price</NuxtLink>.
    </p>
    <ul class="grid">
      <ProductCard v-for="p in data.products" :key="p.slug" :product="p" />
    </ul>
  </section>
</template>
