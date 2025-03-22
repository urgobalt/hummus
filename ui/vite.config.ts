import { BuildOptions, defineConfig, ServerOptions } from "vite";
// @ts-ignore
import tailwindcss from "@tailwindcss/vite";

const build: BuildOptions = {
  outDir: "dist",
  reportCompressedSize: true,
  commonjsOptions: {
    transformMixedEsModules: true,
  },
  emptyOutDir: true,
};

const server: ServerOptions = {
  port: 3000,
};

export default defineConfig({
  build,
  plugins: [tailwindcss()],
  server,
});
