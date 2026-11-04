<script setup lang="ts">
import { PRODUCTS } from "@src/catalog";
import { price, stockLabel } from "@src/format";
import { summary } from "@src/summary";
import ProductCard from "@src/ui/ProductCard.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `product-${route.params.slug}`,
  () => {
    const product = PRODUCTS.find((p) => p.slug === route.params.slug);
    if (!product) throw createError({ statusCode: 404, statusMessage: "Not Found", fatal: true });
    const related = PRODUCTS.filter((p) => p.slug !== product.slug && p.tags[0] === product.tags[0]).slice(0, 4).map(summary);
    return Promise.resolve({ product, related });
  },
);
if (!data.value) throw createError({ statusCode: 404, statusMessage: "Not Found", fatal: true });
</script>

<template>
  <article v-if="data" class="product">
    <h1>{{ data.product.name }}</h1>
    <p class="price">{{ price(data.product.priceCents) }}</p>
    <p class="rating">{{ data.product.rating.toFixed(1) }} / 5</p>
    <p class="stock">{{ stockLabel(data.product.stock) }}</p>
    <p>{{ data.product.blurb }}</p>
    <ul class="tags">
      <li v-for="t in data.product.tags" :key="t">{{ t }}</li>
    </ul>
    <h2>Related</h2>
    <ul class="grid">
      <ProductCard v-for="p in data.related" :key="p.slug" :product="p" />
    </ul>
  </article>
</template>
