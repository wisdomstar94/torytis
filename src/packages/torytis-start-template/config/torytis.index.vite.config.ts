import { join } from "path";
import type { UserConfigFnPromise } from "vite";

const DIRNAME = import.meta.dirname;
const PACKAGE_ROOT = join(DIRNAME, "..");
const OUT_DIR = join(PACKAGE_ROOT, ".torytis");
const SRC_DIR = join(PACKAGE_ROOT, "src/");
const SRC_DIR_CONVERT = SRC_DIR.replace(/\\/g, "/");

const defineConfig: UserConfigFnPromise = async (params) => {
  return {
    root: PACKAGE_ROOT,
    publicDir: false,
    resolve: {
      alias: {
        "@/": SRC_DIR_CONVERT,
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
      preprocessorOptions: {
        scss: {
          api: "modern-compiler",
        },
      },
    },
  };
};

export default defineConfig;
