<script lang="ts">
  import { price } from "@src/format";

  interface Summary {
    slug: string;
    name: string;
    priceCents: number;
    rating: number;
    inStock: boolean;
  }

  let { product }: { product: Summary } = $props();
</script>

<li class="card">
  <a href={`/products/${product.slug}`}><h3>{product.name}</h3></a>
  <p class="price">{price(product.priceCents)}</p>
  <p class="rating">{product.rating.toFixed(1)} / 5</p>
  {#if product.inStock}
    <p class="stock">In stock</p>
  {:else}
    <p class="stock out">Out of stock</p>
  {/if}
</li>
