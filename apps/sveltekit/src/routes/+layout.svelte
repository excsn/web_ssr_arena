<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<svelte:head>
  <title>Outfitters</title>
</svelte:head>

<div class="shell">
  <header class="bar">
    <a href="/" class="brand">Outfitters</a>
    <nav>
      <a href="/products">Products</a>
      <a href="/search?q=pack">Search</a>
      <a href="/counter">Counter</a>
    </nav>
  </header>
  <main class="content">{@render children()}</main>
  <footer class="foot">Outfitters, a benchmark storefront.</footer>
</div>
