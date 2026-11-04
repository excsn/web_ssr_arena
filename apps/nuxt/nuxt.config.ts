import { fileURLToPath } from "node:url";

const cold = Boolean(process.env.ARENA_COLD);

export default defineNuxtConfig({
  compatibilityDate: "2026-11-01",
  telemetry: false,
  devtools: { enabled: false },
  buildDir: cold ? ".nuxt-cold" : ".nuxt",
  css: ["~/assets/app.css"],
  alias: { "@src": fileURLToPath(new URL("./app/src", import.meta.url)) },
  app: { head: { title: "Outfitters", htmlAttrs: { lang: "en" } } },
  routeRules: { "/": { prerender: true }, "/counter": { prerender: true } },
  nitro: { output: { dir: cold ? ".output-cold" : ".output" } },
  vite: { cacheDir: cold ? ".vite-cold" : "node_modules/.vite" },
});
