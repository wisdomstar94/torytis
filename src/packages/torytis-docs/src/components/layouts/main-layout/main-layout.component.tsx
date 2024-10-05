import { Content } from "../content/content.component";
import { Topbar } from "../topbar/topbar.component";
import { Sidebar } from "../sidebar/sidebar.component";
import { IMainLayout } from "./main-layout.type";

export function MainLayout(props: IMainLayout.Props) {
  const { children } = props;

  return (
    <>
      <Topbar />
      <Sidebar />
      <Content>{children}</Content>
    </>
  );
}
