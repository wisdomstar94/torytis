import { join } from "path";
import type { UserConfigFnPromise } from "vite";

const PACKAGE_ROOT = import.meta.dirname;

const defineConfig: UserConfigFnPromise = async (params) => {
  return {
    root: PACKAGE_ROOT,
    publicDir: false,
    resolve: {
      alias: {
        "@/": join(PACKAGE_ROOT, "src/"),
      },
    },
    build: {
      ssr: false,
      sourcemap: false,
      target: `modules`,
      outDir: ".torytis",
      assetsDir: ".",
      minify: false,
      emptyOutDir: false,
      reportCompressedSize: false,
      lib: {
        entry: ".torytis/script.ts",
      },
      rollupOptions: {
        input: ".torytis/script.ts",
        output: [
          // { format: "es", entryFileNames: `script.js` },
          { format: "cjs", entryFileNames: `script.js` },
        ],
      },
    },
    plugins: [],
    css: {
      modules: {
        localsConvention: "dashes",
      },
    },
  };
};

export default defineConfig;
