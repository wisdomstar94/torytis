window.addEventListener("load", () => {
  console.log("loade됨", io);

  const socketUrl = ``;
  const socket = io(`ws://${socketUrl.replace("http://", "").replace("https://", "").replace("ws://", "")}`);
});
