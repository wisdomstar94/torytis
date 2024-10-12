import { join } from "path";
import { build } from "vite";
import { jsx } from "react/jsx-runtime";
import { renderToString } from "react-dom/server";
import fs from "fs";
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

async function indexmjsToSkinhtml() {
  const indexJsx = await import(INDEX_MJS_PATH + `?_=${Date.now()}`);
  const App = indexJsx.default;
  const html = renderToString(jsx(App, {}, Date.now().toString()));
  fs.writeFileSync(SKIN_HTML_PATH, html);
  fs.rmSync(INDEX_MJS_PATH);
  await commandExec(`npm run torytis -- skinhtmlreplace`);
}

async function commandExec(cmd) {
  return new Promise(function (resolve, reject) {
    exec(cmd, (error, stdout, stderr) => {
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

await socketServer(server);
server.listen(SOCKET_PORT);
