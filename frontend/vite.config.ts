import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

export default defineConfig({
  plugins: [vue()],
  optimizeDeps: { include: ["firebase/app", "firebase/auth"] },
  alias: {
    "@": path.resolve(__dirname, "."),
  },
});
