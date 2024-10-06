import { MainLayout } from "@/components/layouts/main-layout/main-layout.component";
import { SIDEBAR_MENU_ITEMS } from "@/consts/sidebar-menu-list/sidebar-menu-items.const";
import Intro from "@/markdown/intro.mdx";

export default function Page() {
  return (
    <>
      <MainLayout activeKeys={[SIDEBAR_MENU_ITEMS.find((x) => x.key === "/intro")?.key ?? ""]}>
        <Intro />
      </MainLayout>
    </>
  );
}
