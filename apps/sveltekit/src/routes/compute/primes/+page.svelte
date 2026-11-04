<script lang="ts">
  import PrimesView from "@src/views/Primes.svelte";

  let { data } = $props();
</script>

<PrimesView result={data.result} />
