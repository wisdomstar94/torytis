import { ISidebar } from "@/components/layouts/sidebar/sidebar.type";

export const SIDEBAR_MENU_ITEMS = [
  {
    key: "/intro" as const,
    href: "/intro",
    label: "torytis 소개",
    activeRegExpItems: [`/intro.*`],
  },
  {
    key: "/installation" as const,
    href: "/installation",
    label: "torytis 설치 방법",
    activeRegExpItems: [`/installation.*`],
  },
] satisfies ISidebar.MenuItem[];
