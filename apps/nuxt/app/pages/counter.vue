<script setup lang="ts">
import Counter from "@src/ui/Counter.vue";
</script>

<template>
  <section>
    <h1>Counter</h1>
    <p>One component that runs in the browser.</p>
    <Counter :start="3" />
  </section>
</template>
