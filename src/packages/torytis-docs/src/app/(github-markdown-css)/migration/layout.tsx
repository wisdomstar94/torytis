import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 업데이트 가이드",
  description: "torytis 를 업데이트 하는 가이드를 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
