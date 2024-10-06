import { Content } from "../content/content.component";
import { Topbar } from "../topbar/topbar.component";
import { Sidebar } from "../sidebar/sidebar.component";
import { IMainLayout } from "./main-layout.type";
import { SIDEBAR_MENU_ITEMS } from "@/consts/sidebar-menu-list/sidebar-menu-items.const";

export function MainLayout(props: IMainLayout.Props) {
  const { children, activeKeys } = props;

  return (
    <>
      <Topbar />
      <Sidebar activeKeys={activeKeys} menuItems={SIDEBAR_MENU_ITEMS} />
      <Content>{children}</Content>
    </>
  );
}
