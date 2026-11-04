<script lang="ts">
  import type { PrimeSummary } from "@src/compute";

  let { result }: { result: PrimeSummary } = $props();
</script>

<section>
  <h1>Primes up to {result.n}</h1>
  <p class="digest">{`primes ${result.n} ${result.count} ${result.sum} ${result.last.join(",")}`}</p>
  <ul class="tags">
    {#each result.last as p (p)}
      <li>{p}</li>
    {/each}
  </ul>
</section>
