<script lang="ts">
  const FEATURES = [
    { title: "Built for the trail", body: "Every pack, jacket, lantern and bottle is tested over a full season before it is listed." },
    { title: "Repairs for life", body: "Send anything back with a tear or a broken buckle and it comes home fixed." },
    { title: "Shipped in a day", body: "Orders placed before noon leave the warehouse the same afternoon." },
  ];
</script>

<section class="hero">
  <h1>Gear for long days out</h1>
  <p>Forty-eight things we carry ourselves, from packs to lanterns.</p>
  <a href="/products" class="cta">Browse the catalog</a>
</section>
<section class="features">
  {#each FEATURES as f (f.title)}
    <article>
      <h2>{f.title}</h2>
      <p>{f.body}</p>
    </article>
  {/each}
</section>
