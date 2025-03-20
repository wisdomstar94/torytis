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
      preprocessorOptions: {
        scss: {
          api: "modern-compiler",
        },
      },
    },
  };
};

export default defineConfig;
