window.addEventListener("load", () => {
  console.log("loade됨", io);

  const SOCKET_PORT = 3008;
  const socketUrl = `localhost:${SOCKET_PORT}`;
  const socket = io(`ws://${socketUrl.replace("http://", "").replace("https://", "").replace("ws://", "")}`);

  socket.on("full-reload", () => {
    location.reload();
  });
});
