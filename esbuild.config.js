const esbuild = require("esbuild");

esbuild.build({
  entryPoints: ["src-cli/torytis.ts"],
  target: ['es5', 'es6', 'es2017'],
  bundle: true,
  sourcemap: false,
  minify: true,
  format: 'cjs',
  platform: 'node',
  outfile: "bin/torytis.js",
  external: ['esbuild', 'postcss-modules'],
  treeShaking: true,
  plugins: [],
});