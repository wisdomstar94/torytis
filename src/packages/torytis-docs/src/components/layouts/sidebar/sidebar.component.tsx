"use client";

import { sidebarToggle } from "@/functions/common";
import { cn } from "@/utils/common";

export function Sidebar() {
  return (
    <>
      <div
        className={cn(
          "flex lg:hidden fixed top-0 left-0 z-[2] bg-black/70 overflow-hidden",
          "sidebar-toggle-yes:animate-fade-in-opacity sidebar-toggle-no:animate-fade-out-opacity"
          //
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

          // "left-0",
          "-left-sidebar-width sidebar-toggle-yes:left-0",
          "lg:left-0 lg:sidebar-toggle-yes:-left-sidebar-width",

          // "sidebar-collapsed:-left-sidebar-width",
          // "sidebar-collapsed:shadow-transparent",

          "bg-slate-50 shadow-[2px_2px_6px]",

          "shadow-transparent lg:shadow-slate-200 lg:sidebar-toggle-yes:shadow-transparent"
          //
        )}
      >
        sidebar
      </aside>
    </>
  );
}
