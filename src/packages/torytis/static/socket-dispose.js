window.addEventListener("load", () => {
  // console.log("loade됨", io);
  const firstConnectTime = Date.now();
  let latestConnectTime = Date.now();

  const SOCKET_PORT = 3020;
  const socketUrl = `localhost:${SOCKET_PORT}`;
  const socket = io(`ws://${socketUrl.replace("http://", "").replace("https://", "").replace("ws://", "")}`);

  socket.on("connect", () => {
    latestConnectTime = Date.now();

    if (latestConnectTime - firstConnectTime >= 1000 * 5) {
      location.reload();
    }
  });

  socket.on("full-reload", () => {
    location.reload();
  });
});
