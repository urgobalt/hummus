import { defineConfig } from "@rsbuild/core";

export default defineConfig({
  source: {
    entry: {
      index: "./src/main.ts",
    },
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
