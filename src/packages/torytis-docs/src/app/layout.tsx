import type { Metadata } from "next";
import "../styles/globals.scss";

export const metadata: Metadata = {
  title: "torytis - home",
  description: "torytis 공식 문서 사이트 입니다.",
  icons: {
    icon: "/torytis/favicon.ico",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ko">
      <body id="body">{children}</body>
    </html>
  );
}
