<script lang="ts">
  import ProductCard from "@src/ui/ProductCard.svelte";

  let { data } = $props();
</script>

<section>
  <h1>Search</h1>
  <form action="/search" method="get">
    <input type="search" name="q" value={data.q} />
    <button type="submit">Search</button>
  </form>
  <p>{data.results.length} results for &quot;{data.q}&quot;</p>
  <ul class="grid">
    {#each data.results as p (p.slug)}
      <ProductCard product={p} />
    {/each}
  </ul>
</section>
