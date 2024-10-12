import { join } from "path";
import { build } from "vite";
import { jsx } from "react/jsx-runtime";
import { renderToString } from "react-dom/server";
import fs from "fs";
import os from "os";
import { exec } from "child_process";

import express from "express";
import { Server } from "socket.io";
import http from "http";

const DIRNAME = import.meta.dirname;
const PACKAGE_ROOT = join(DIRNAME, "..");
const INDEX_MJS_PATH = join(PACKAGE_ROOT, ".torytis", "index.mjs");
const SKIN_HTML_PATH = join(PACKAGE_ROOT, ".torytis", "skin.html");
const SOCKET_PORT = 3008;

const app = express();
const server = http.createServer(app);

async function socketServer(
  /** @type {import('http').Server} */
  server
) {
  await commandExec(`npm run torytis -- buildpreprocess`);

  const io = new Server(server, {
    cors: {
      origin: "*",
      methods: ["GET", "POST"],
    },
  });

  io.on("connection", async (socket) => {});

  const mounted = {
    index: false,
    script: false,
  };

  const isAllMounted = () => {
    return mounted.index && mounted.script;
  };

  // html, css 처리
  await build({
    configFile: join(PACKAGE_ROOT, "config", "torytis.index.vite.config.ts"),
    build: {
      watch: {
        include: join(PACKAGE_ROOT, "src/**/*"),
        exclude: join(PACKAGE_ROOT, "src/**/*.script.tsx"),
      },
    },
    plugins: [
      {
        name: "on-change",
        buildStart: async () => {
          await commandExec(`npm run torytis -- movepublictodottorytis`);
        },
        closeBundle: async () => {
          await indexmjsToSkinhtml();
          mounted.index = true;
          const allMounted = isAllMounted();
          if (allMounted) {
            io.emit("full-reload");
          } else {
            await commandExec(`npm run torytis -- scriptbundle`);
          }
        },
      },
    ],
  });

  // script 처리
  await build({
    configFile: join(PACKAGE_ROOT, "config", "torytis.script.vite.config.ts"),
    build: {
      watch: {
        include: join(PACKAGE_ROOT, "src/**/*.script.tsx"),
      },
    },
    plugins: [
      {
        name: "on-change",
        buildStart: async () => {
          if (isAllMounted()) {
            await commandExec(`npm run torytis -- scriptbundle`);
          }
        },
        closeBundle: async () => {
          await commandExec(`npm run torytis -- scriptpostprocess`);
          mounted.script = true;
          const allMounted = isAllMounted();
          if (allMounted) {
            io.emit("full-reload");
          }
        },
      },
    ],
  });
}

function isWindow() {
  const osType = os.type();
  return osType.toLowerCase().includes("window");
}

async function indexmjsToSkinhtml() {
  const prefix = isWindow() ? "file://" : "";
  const indexJsx = await import(prefix + INDEX_MJS_PATH + `?_=${Date.now()}`);
  const App = indexJsx.default;
  const html = renderToString(jsx(App, {}, Date.now().toString()));
  fs.writeFileSync(SKIN_HTML_PATH, html);
  fs.rmSync(INDEX_MJS_PATH);
  await commandExec(`npm run torytis -- skinhtmlreplace`);
}

async function commandExec(cmd) {
  const shell = isWindow() ? "cmd.exe" : "bash";

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
    // try {
    //   const target = spawn(shell);
    //   target.stdin.write(cmd);
    //   target.stdin.end();
    //   target.on("message", function (message) {
    //     console.log("@message", message);
    //   });
    //   target.on("close", function (code) {
    //     // console.log('end');
    //     resolve(code);
    //   });
    // } catch (e) {
    //   console.error(`\n[ error ]\n`, e);
    //   resolve(e);
    // }
  });
}

await socketServer(server);
server.listen(SOCKET_PORT);
