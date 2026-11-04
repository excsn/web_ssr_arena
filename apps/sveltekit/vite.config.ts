import { fileURLToPath } from "node:url";
import adapter from "@sveltejs/adapter-node";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

const cold = Boolean(process.env.ARENA_COLD);

export default defineConfig({
  plugins: [sveltekit({ adapter: adapter({ out: cold ? "build-cold" : "build" }), outDir: cold ? ".svelte-kit-cold" : ".svelte-kit" })],
  resolve: { alias: { "@src": fileURLToPath(new URL("./src/lib", import.meta.url)) } },
  cacheDir: cold ? ".vite-cold" : "node_modules/.vite",
});
