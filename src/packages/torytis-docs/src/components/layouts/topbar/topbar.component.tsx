"use client";

import { sidebarToggle } from "@/functions/common";
import { cn } from "@/utils/common";
import { Menu } from "lucide-react";

export function Topbar() {
  return (
    <header
      className={cn(
        "transition-all duration-300",
        // "w-full-subtraction-sidebar-width",
        "w-full",
        "lg:w-full-subtraction-sidebar-width lg:sidebar-toggle-yes:w-full",

        "h-topbar-height",
        "z-[1]",

        "fixed top-0 right-0",
        "bg-slate-50 shadow-slate-200 shadow-[2px_2px_4px]",
        "flex gap-2 items-center justify-between"
        //
      )}
    >
      {/* left area */}
      <div className="flex-shrink-0 flex-grow-0 relative inline-flex gap-2">
        <button
          className="px-4 py-2 cursor-pointer"
          onClick={() => {
            sidebarToggle();
          }}
        >
          <Menu />
        </button>
      </div>

      {/* right area */}
      <div className="flex-shrink-0 flex-grow-0 relative inline-flex gap-2"></div>
    </header>
  );
}
