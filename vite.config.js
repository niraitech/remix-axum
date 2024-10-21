import { vitePlugin as remix } from "@remix-run/dev";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [remix()],
  server: {
    proxy: {
      "/api": "http://localhost:4444",
      "/auth": "http://localhost:4444",
    },
  },
});
