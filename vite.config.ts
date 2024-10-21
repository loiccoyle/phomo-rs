import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  base: "/phomo-rs/",
  plugins: [react(), wasm(), topLevelAwait()],
  worker: { plugins: () => [wasm(), topLevelAwait()], format: "es" },
});
