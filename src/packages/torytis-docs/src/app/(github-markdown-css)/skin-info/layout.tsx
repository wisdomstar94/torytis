import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 스킨 정보 편집",
  description: "torytis 에서 스킨에 대한 정보를 편집하는 방법에 대해 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
