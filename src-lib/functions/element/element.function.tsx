import { ReactElement } from "react";
import { renderToString } from "react-dom/server"

export function jsxToHtmlString(jsx: ReactElement) {
  return renderToString(jsx);
}