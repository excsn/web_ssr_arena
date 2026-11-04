<script lang="ts">
  import type { Words } from "@src/compute";

  let { result }: { result: Words } = $props();
</script>

<section>
  <h1>Word counts</h1>
  <p class="digest">{`words ${result.total} ${result.distinct} ${result.top.map((w) => `${w.word}:${w.count}`).join(",")}`}</p>
  <ol>
    {#each result.top as w (w.word)}
      <li>{w.word} {w.count}</li>
    {/each}
  </ol>
</section>
