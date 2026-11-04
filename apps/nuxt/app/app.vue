<template>
  <div class="shell">
    <header class="bar">
      <NuxtLink to="/" class="brand">Outfitters</NuxtLink>
      <nav>
        <NuxtLink to="/products">Products</NuxtLink>
        <NuxtLink to="/search?q=pack">Search</NuxtLink>
        <NuxtLink to="/counter">Counter</NuxtLink>
      </nav>
    </header>
    <main class="content"><NuxtPage /></main>
    <footer class="foot">Outfitters, a benchmark storefront.</footer>
  </div>
</template>
