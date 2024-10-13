import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 실제 사용 사례",
  description: "torytis 를 사용하여 스킨을 개발한 실제 사용 사례를 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
