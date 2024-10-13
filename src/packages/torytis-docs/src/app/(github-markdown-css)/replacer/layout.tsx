import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "torytis - 치환자 소개",
  description: "torytis 에서 제공하는 치환자를 소개합니다.",
};

export default function Layout({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
