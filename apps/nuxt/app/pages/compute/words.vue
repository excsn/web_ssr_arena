<script setup lang="ts">
import { words } from "@src/compute";
import WordsView from "@src/views/Words.vue";

const route = useRoute();
const { data } = await useAsyncData(
  () => `words-${route.query.passes}`,
  () => Promise.resolve(words(Math.min(Math.max(Number(route.query.passes ?? "20") || 0, 1), 1000))),
);
</script>

<template>
  <WordsView v-if="data" :result="data" />
</template>
