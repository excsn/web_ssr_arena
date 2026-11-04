<script setup lang="ts">
const props = defineProps<{ start: number }>();
const count = ref(props.start);
</script>

<template>
  <div class="counter">
    <button type="button" @click="count -= 1">-</button>
    <output>{{ count }}</output>
    <button type="button" @click="count += 1">+</button>
  </div>
</template>
