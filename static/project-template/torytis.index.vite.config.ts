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
      ssr: true,
      sourcemap: false,
      target: `modules`,
      outDir: ".torytis",
      assetsDir: ".",
      minify: true,
      emptyOutDir: false,
      reportCompressedSize: false,
      lib: {
        entry: "src/index.component.tsx",
      },
      rollupOptions: {
        input: "src/index.component.tsx",
        output: [
          { format: "es", entryFileNames: `index.mjs` },
          // { format: "cjs", entryFileNames: `[name].cjs` },
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
