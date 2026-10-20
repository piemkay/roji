import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

// The dev server forwards /api to the Roji server. scripts/dev sets ROJI_API_URL to the
// server of the current worktree.
const apiUrl = process.env.ROJI_API_URL ?? "http://127.0.0.1:8080";

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: { "/api": apiUrl },
  },
  test: {
    environment: "jsdom",
  },
});
