window.addEventListener("load", () => {
  const helloWorldDiv = document.querySelector<HTMLElement>("#hello-world");
  if (helloWorldDiv === null) return;
  helloWorldDiv.style.border = "2px solid #00f";
});
