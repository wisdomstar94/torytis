import { exec } from "child_process";
import { join } from "path";
import jsx_runtime from "react/jsx-runtime";
import { renderToString } from "react-dom/server";
import fs from "fs";
import os from "os";

const DIRNAME = import.meta.dirname;
const PACKAGE_ROOT = join(DIRNAME, "..");

const disposeIndexComponent = async () => {
  await commandExec(`npm run vite -- --config "${join(PACKAGE_ROOT, "config", "torytis.index.vite.config.ts").replace(/\\/g, "/")}" build`);

  const indexJsxPath = join(PACKAGE_ROOT, ".torytis", "index.mjs");
  const skinHtmlPath = join(PACKAGE_ROOT, ".torytis", "skin.html");

  const prefix = isWindow() ? "file://" : "";
  const indexJsx = await import(prefix + indexJsxPath);
  const App = indexJsx.default;
  const html = renderToString(jsx_runtime.jsx(App, {}));
  fs.writeFileSync(skinHtmlPath, html);
  fs.rmSync(indexJsxPath);
};

const disposeIndexScript = async () => {
  return await commandExec(
    `npm run vite -- --config ${join(PACKAGE_ROOT, "config", "torytis.script.vite.config.ts").replace(/\\/g, "/")} build`
  );
};

(async () => {
  const start = Date.now();
  console.log("torytis building...");
  await Promise.all([disposeIndexComponent(), disposeIndexScript()]);

  const end = Date.now();
  console.log(`torytis build success! [${end - start} ms]`);
})();

function isWindow() {
  const osType = os.type();
  return osType.toLowerCase().includes("window");
}

async function commandExec(cmd) {
  const osType = os.type();
  const shell = osType.toLowerCase().includes("window") ? "cmd.exe" : "bash";

  return new Promise(function (resolve, reject) {
    exec(cmd, { shell }, (error, stdout, stderr) => {
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

      resolve(stdout);
    });
  });
}
