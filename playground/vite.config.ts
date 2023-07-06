import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import rust from "@wasm-tool/rollup-plugin-rust";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    rust({
      verbose: true,
      experimental: {
        directExports: true,
      },
    }),
    topLevelAwait(),
  ],
});
