<script setup lang="ts">
import { price } from "@src/format";

defineProps<{ product: { slug: string; name: string; priceCents: number; rating: number; inStock: boolean } }>();
</script>

<template>
  <li class="card">
    <NuxtLink :to="`/products/${product.slug}`"><h3>{{ product.name }}</h3></NuxtLink>
    <p class="price">{{ price(product.priceCents) }}</p>
    <p class="rating">{{ product.rating.toFixed(1) }} / 5</p>
    <p v-if="product.inStock" class="stock">In stock</p>
    <p v-else class="stock out">Out of stock</p>
  </li>
</template>
