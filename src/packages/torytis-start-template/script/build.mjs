import { exec } from "child_process";
import { join } from "path";
import jsx_runtime from "react/jsx-runtime";
import { renderToString } from "react-dom/server";
import fs from "fs";

const DIRNAME = import.meta.dirname;
const PACKAGE_ROOT = join(DIRNAME, "..");

const disposeIndexComponent = async () => {
  await new Promise(function (resolve, reject) {
    const command = `npm run vite -- --config ${join(PACKAGE_ROOT, "config", "torytis.index.vite.config.ts")} build`;

    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error("\n[error]\n", error);
        resolve(error);
        return;
      }
      if (stderr) {
        console.error("\n[stderr]\n", stderr);
        resolve(stderr);
        return;
      }

      console.log("@stdout", stdout);
      resolve(stdout);
    });
  });

  const indexJsxPath = join(PACKAGE_ROOT, ".torytis", "index.mjs");
  const skinHtmlPath = join(PACKAGE_ROOT, ".torytis", "skin.html");

  const indexJsx = await import(indexJsxPath);
  const App = indexJsx.default;
  const html = renderToString(jsx_runtime.jsx(App, {}));
  fs.writeFileSync(skinHtmlPath, html);
  fs.rmSync(indexJsxPath);
};

const disposeIndexScript = async () => {
  await new Promise(function (resolve, reject) {
    const command = `npm run vite -- --config ${join(PACKAGE_ROOT, "config", "torytis.script.vite.config.ts")} build`;

    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error("\n[error]\n", error);
        resolve(error);
        return;
      }
      if (stderr) {
        console.error("\n[stderr]\n", stderr);
        resolve(stderr);
        return;
      }

      console.log("@stdout", stdout);
      resolve(stdout);
    });
  });
};

(async () => {
  const start = Date.now();
  console.log("torytis building...");
  await Promise.all([disposeIndexComponent(), disposeIndexScript()]);

  const end = Date.now();
  console.log(`torytis build success! [${end - start} ms]`);
})();
