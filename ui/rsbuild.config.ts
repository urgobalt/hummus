import { defineConfig } from "@rsbuild/core";

export default defineConfig({
  source: {
    entry: {
      index: "./src/main.ts",
    },
  },
  server: {
    port: parseInt(process.env.PORT ?? "3000"),
    host: process.env.HOST ?? "localhost",
  },
  output: {
    cleanDistPath: true,
    distPath: {
      root: "dist",
      css: "assets",
      js: "assets",
    },
  },
  tools: {
    htmlPlugin: {
      template: "./src/index.html",
    },
  },
});
