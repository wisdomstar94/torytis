"use client";

import { sidebarToggle } from "@/functions/common";
import { cn } from "@/utils/common";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { ISidebar } from "./sidebar.type";

export function Sidebar<T extends string>(props: ISidebar.Props<T>) {
  const { menuItems } = props;
  const [activeKeys, setActiveKeys] = useState<Set<string>>(new Set(props.activeKeys));
  const pathname = usePathname();

  useEffect(() => {
    for (const item of menuItems) {
      if (!Array.isArray(item.activeRegExpItems)) continue;

      for (const regString of item.activeRegExpItems) {
        const reg = new RegExp(regString);
        if (reg.test(pathname)) {
          setActiveKeys((prev) => {
            const newPrev = new Set(prev);
            newPrev.add(item.key);
            return newPrev;
          });
        } else {
          setActiveKeys((prev) => {
            const newPrev = new Set(prev);
            newPrev.delete(item.key);
            return newPrev;
          });
        }
      }
    }
  }, [pathname, menuItems]);

  return (
    <>
      <div
        className={cn(
          "flex lg:hidden fixed top-0 left-0 z-[2] bg-black/70 overflow-hidden",
          "sidebar-toggle-yes:animate-fade-in-opacity sidebar-toggle-no:animate-fade-out-opacity"
        )}
        onClick={() => {
          sidebarToggle();
        }}
      />

      <aside
        className={cn(
          "transition-all duration-300",
          "w-sidebar-width",
          "h-full",

          "fixed top-0 overflow-y-auto",
          "z-[2]",

          "-left-sidebar-width sidebar-toggle-yes:left-0",
          "lg:left-0 lg:sidebar-toggle-yes:-left-sidebar-width",

          "bg-slate-50 shadow-[2px_2px_6px]",

          "shadow-transparent lg:shadow-slate-200 lg:sidebar-toggle-yes:shadow-transparent"
        )}
      >
        {/* logo area */}
        <div className="w-full relative box-borde px-4 py-8 flex items-center justify-center font-extralight text-xl">torytis</div>

        {/* menu list */}
        <ul className="w-full flex flex-wrap gap-1 relative">
          {menuItems.map((item) => {
            return (
              <li key={item.key} className="w-full flex gap-2 relative text-slate-500">
                <Link
                  className={cn("w-full px-4 py-1 text-sm", activeKeys.has(item.key) && "my-active", "my-active:text-slate-900")}
                  href={item.href}
                >
                  {item.label}
                </Link>
              </li>
            );
          })}
        </ul>
      </aside>
    </>
  );
}
