import { join } from "path";
import { build } from "vite";
import { jsx } from "react/jsx-runtime";
import { renderToString } from "react-dom/server";
import fs from "fs";
import os from "os";

import express from "express";
import { Server } from "socket.io";
import http from "http";

const DIRNAME = import.meta.dirname;
const PACKAGE_ROOT = join(DIRNAME, "..");
const INDEX_MJS_PATH = join(PACKAGE_ROOT, ".torytis", "index.mjs");
const SKIN_HTML_PATH = join(PACKAGE_ROOT, ".torytis", "skin.html");
const SOCKET_PORT = 3020;
const DEV_SERVER_PORT = 3000;
const DEV_SERVER_BASE_URL = `http://localhost:${DEV_SERVER_PORT}`;

const app = express();
const server = http.createServer(app);

async function socketServer(
  /** @type {import('http').Server} */
  server
) {
  await fetch(`${DEV_SERVER_BASE_URL}/processer/buildpreprocess`);

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
          await fetch(`${DEV_SERVER_BASE_URL}/processer/movepublictodottorytis`);
        },
        closeBundle: async () => {
          await indexmjsToSkinhtml();
          mounted.index = true;
          const allMounted = isAllMounted();
          if (allMounted) {
            io.emit("full-reload");
          } else {
            await fetch(`${DEV_SERVER_BASE_URL}/processer/scriptbundle`);
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
            await fetch(`${DEV_SERVER_BASE_URL}/processer/scriptbundle`);
          }
        },
        closeBundle: async () => {
          await fetch(`${DEV_SERVER_BASE_URL}/processer/scriptpostprocess`);
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
  await fetch(`${DEV_SERVER_BASE_URL}/processer/skinhtmlreplace`);
}

await socketServer(server);
server.listen(SOCKET_PORT);
