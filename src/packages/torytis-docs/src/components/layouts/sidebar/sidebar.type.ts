import { ReactNode } from "react";

export declare namespace ISidebar {
  export type MenuItem<T extends string = string> = {
    key: T;
    label: ReactNode;
    href: string;
    children?: MenuItem<T>[];
    activeRegExpItems?: string[];
  };

  export type Props<T extends string> = {
    activeKeys: string[];
    menuItems: MenuItem<T>[];
  };
}
