<script lang="ts">
  import { price, stockLabel } from "@src/format";
  import ProductCard from "@src/ui/ProductCard.svelte";

  let { data } = $props();
</script>

<article class="product">
  <h1>{data.product.name}</h1>
  <p class="price">{price(data.product.priceCents)}</p>
  <p class="rating">{data.product.rating.toFixed(1)} / 5</p>
  <p class="stock">{stockLabel(data.product.stock)}</p>
  <p>{data.product.blurb}</p>
  <ul class="tags">
    {#each data.product.tags as t (t)}
      <li>{t}</li>
    {/each}
  </ul>
  <h2>Related</h2>
  <ul class="grid">
    {#each data.related as p (p.slug)}
      <ProductCard product={p} />
    {/each}
  </ul>
</article>
