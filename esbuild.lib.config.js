const esbuild = require("esbuild");

esbuild.build({
  entryPoints: ["index.ts"],
  target: ['es5', 'es6', 'es2017'],
  bundle: true,
  sourcemap: false,
  minify: true,
  format: 'cjs',
  platform: 'browser',
  outfile: "index.js",
  external: ['esbuild', 'postcss-modules', 'react', 'react-dom'],
  treeShaking: true,
  tsconfig: "./tsconfig.package.json",
});