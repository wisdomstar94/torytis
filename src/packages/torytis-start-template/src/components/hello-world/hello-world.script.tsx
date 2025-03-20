window.addEventListener("load", () => {
  const helloWorldDiv = document.querySelector<HTMLElement>("#hello-world");
  if (helloWorldDiv === null) return;
  console.log(`...helloWorldDiv`, helloWorldDiv);
});
