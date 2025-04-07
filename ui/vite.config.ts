import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import tailwind from "@tailwindcss/vite";

export default defineConfig({
  plugins: [
    vue(),
    AutoImport({
      imports: [
        "vue",
        {
          "@unhead/vue": ["useHead"],
        },
      ],
      dts: "auto-imports.d.ts",
      vueTemplate: true,
    }),
    Components({
      dts: "components.d.ts",
    }),
    tailwind(),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
