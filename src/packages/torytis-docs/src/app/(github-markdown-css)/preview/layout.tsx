import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 개발 결과 미리보기",
  description: "torytis 개발 결과를 미리보는 방법을 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
