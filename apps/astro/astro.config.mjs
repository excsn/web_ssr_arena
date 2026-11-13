import { fileURLToPath } from "node:url";
import node from "@astrojs/node";
import react from "@astrojs/react";
import { defineConfig } from "astro/config";

const cold = Boolean(process.env.ARENA_COLD);

export default defineConfig({
  output: "server",
  adapter: node({ mode: "standalone" }),
  integrations: [react()],
  outDir: cold ? "dist-cold" : "dist",
  cacheDir: cold ? ".astro-cold" : "node_modules/.astro",
  vite: {
    resolve: { alias: { "@src": fileURLToPath(new URL("./src/lib", import.meta.url)) } },
    cacheDir: cold ? ".vite-cold" : "node_modules/.vite",
    ssr: { noExternal: true },
  },
});
