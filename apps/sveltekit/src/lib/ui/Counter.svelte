<script lang="ts">
  let { start }: { start: number } = $props();
  let count = $state(start);
</script>

<div class="counter">
  <button type="button" onclick={() => (count -= 1)}>-</button>
  <output>{count}</output>
  <button type="button" onclick={() => (count += 1)}>+</button>
</div>
