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
    key: "/skin-info" as const,
    href: "/skin-info",
    label: "torytis 스킨 정보 편집",
    activeRegExpItems: [`/skin-info.*`],
  },
  {
    key: "/skin-option-variable-build" as const,
    href: "/skin-option-variable-build",
    label: "torytis 스킨 옵션 변수 빌드",
    activeRegExpItems: [`/skin-option-variable-build.*`],
  },
  {
    key: "/build" as const,
    href: "/build",
    label: "torytis 빌드",
    activeRegExpItems: [`/build.*`],
  },
  {
    key: "/preview" as const,
    href: "/preview",
    label: "torytis 개발 결과물 미리보기",
    activeRegExpItems: [`/preview.*`],
  },
  {
    key: "/migration" as const,
    href: "/migration",
    label: "torytis 버전 업데이트 가이드",
    activeRegExpItems: [`/migration.*`],
  },
  {
    key: "/real-case" as const,
    href: "/real-case",
    label: "torytis 실제 사용 사례",
    activeRegExpItems: [`/real-case.*`],
  },
] satisfies ISidebar.MenuItem[];
