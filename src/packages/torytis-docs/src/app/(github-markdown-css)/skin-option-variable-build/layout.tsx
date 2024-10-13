import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 스킨 옵션 변수 빌드",
  description: "torytis 에서 스킨 옵션 변수를 빌드하는 방법에 대해 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
