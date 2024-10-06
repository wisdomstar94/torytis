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
  {
    key: "/feature" as const,
    href: "/feature",
    label: "torytis 특징",
    activeRegExpItems: [`/feature.*`],
  },
  {
    key: "/replacer" as const,
    href: "/replacer",
    label: "torytis 치환자 소개",
    activeRegExpItems: [`/replacer.*`],
  },
  {
    key: "/preview" as const,
    href: "/preview",
    label: "torytis 개발 결과물 미리보기",
    activeRegExpItems: [`/preview.*`],
  },
] satisfies ISidebar.MenuItem[];
