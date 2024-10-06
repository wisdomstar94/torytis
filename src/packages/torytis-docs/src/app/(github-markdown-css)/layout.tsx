import "github-markdown-css";
import { ReactNode } from "react";

export default function Layout(props: { children: ReactNode }) {
  return <>{props.children}</>;
}
