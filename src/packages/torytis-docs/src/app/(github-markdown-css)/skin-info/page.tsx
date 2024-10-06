import { MainLayout } from "@/components/layouts/main-layout/main-layout.component";
import { SIDEBAR_MENU_ITEMS } from "@/consts/sidebar-menu-items/sidebar-menu-items.const";
import SkinInfo from "@/markdown/skin-info.mdx";

export default function Page() {
  return (
    <>
      <MainLayout activeKeys={[SIDEBAR_MENU_ITEMS.find((x) => x.key === "/skin-info")?.key ?? ""]}>
        <SkinInfo />
      </MainLayout>
    </>
  );
}
