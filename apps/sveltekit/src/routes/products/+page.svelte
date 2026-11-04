<script lang="ts">
  import ProductCard from "@src/ui/ProductCard.svelte";

  let { data } = $props();
</script>

<section>
  <h1>All products</h1>
  <p>
    {data.products.length} products by {data.sort}. Sort by <a href="/products?sort=rating">rating</a> or <a href="/products?sort=price">price</a>.
  </p>
  <ul class="grid">
    {#each data.products as p (p.slug)}
      <ProductCard product={p} />
    {/each}
  </ul>
</section>
