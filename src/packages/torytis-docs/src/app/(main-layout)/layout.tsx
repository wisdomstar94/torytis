import "github-markdown-css";
import { MainLayout } from "@/components/layouts/main-layout/main-layout.component";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <MainLayout>{children}</MainLayout>
    </>
  );
}
