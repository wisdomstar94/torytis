import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 특징",
  description: "torytis 특징을 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
