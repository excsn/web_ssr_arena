#!/usr/bin/env bash
# Removes everything the arena writes: Rust targets through `cargo clean`, generated FSR files,
# Node build output and `runs/`. Without --keep-installs it also removes `node_modules/`, FSR's
# fetched types and `tools/`, which holds `dx` and the wasm-bindgen, esbuild and binaryen it downloads.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
keep_installs=false
case "${1:-}" in
  "") ;;
  --keep-installs) keep_installs=true ;;
  *)
    echo "usage: scripts/clean.sh [--keep-installs]" >&2
    exit 2
    ;;
esac

cargo_clean() {
  local manifest=$1 target=$2
  if [ -d "$target" ]; then
    echo "cargo clean ${target#"$root"/}"
    cargo clean --quiet --manifest-path "$manifest" --target-dir "$target"
  fi
}

remove() {
  local path
  for path in "$@"; do
    if [ -e "$path" ]; then
      echo "remove ${path#"$root"/}"
      rm -rf "$path"
    fi
  done
}

cargo_clean "$root/Cargo.toml" "$root/target"
cargo_clean "$root/apps/fsr/Cargo.toml" "$root/apps/fsr/target"
cargo_clean "$root/apps/leptos/Cargo.toml" "$root/apps/leptos/target"
cargo_clean "$root/apps/dioxus/Cargo.toml" "$root/apps/dioxus/target"
cargo_clean "$root/apps/fsr/Cargo.toml" "$root/runs/cold-fsr-target"
cargo_clean "$root/apps/leptos/Cargo.toml" "$root/runs/cold-leptos-target"
cargo_clean "$root/apps/dioxus/Cargo.toml" "$root/runs/cold-dioxus-target"

remove "$root/runs"
remove "$root/apps/fsr/app/.fsr" "$root/apps/fsr/app/dist" "$root/apps/fsr/app/generated" "$root/apps/fsr/app/tsconfig.json" "$root/apps/fsr/app/tsconfig.build.json"
remove "$root/apps/nextjs/.next" "$root/apps/nextjs/.next-cold" "$root/apps/nextjs/next-env.d.ts"
remove "$root/apps/sveltekit/.svelte-kit" "$root/apps/sveltekit/.svelte-kit-cold" "$root/apps/sveltekit/build" "$root/apps/sveltekit/build-cold" "$root/apps/sveltekit/.vite-cold"
remove "$root/apps/nuxt/.nuxt" "$root/apps/nuxt/.nuxt-cold" "$root/apps/nuxt/.output" "$root/apps/nuxt/.output-cold" "$root/apps/nuxt/.vite-cold"

if [ "$keep_installs" = false ]; then
  remove "$root/apps/nextjs/node_modules" "$root/apps/sveltekit/node_modules" "$root/apps/nuxt/node_modules"
  remove "$root/apps/fsr/app/types"
  remove "$root/tools"
fi
