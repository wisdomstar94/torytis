export function getCategoryNameFromAElement(aElement: HTMLAnchorElement) {
  const cloneA = aElement.cloneNode(true) as HTMLAnchorElement;
  cloneA.querySelector("span")?.remove();
  return cloneA.textContent?.trim();
}
