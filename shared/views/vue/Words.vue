<script setup lang="ts">
import type { Words } from "@src/compute";

defineProps<{ result: Words }>();
</script>

<template>
  <section>
    <h1>Word counts</h1>
    <p class="digest">{{ `words ${result.total} ${result.distinct} ${result.top.map((w) => `${w.word}:${w.count}`).join(",")}` }}</p>
    <ol>
      <li v-for="w in result.top" :key="w.word">{{ w.word }} {{ w.count }}</li>
    </ol>
  </section>
</template>
