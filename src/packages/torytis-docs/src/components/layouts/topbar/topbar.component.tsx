"use client";

import { sidebarToggle } from "@/functions/common";
import { cn } from "@/utils/common";
import { Menu } from "lucide-react";
import Link from "next/link";

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
      <div className="flex-shrink-0 flex-grow-0 relative inline-flex gap-1 pr-4">
        <Link className="p-1 cursor-pointer rounded-full hover:bg-slate-100" href="https://www.npmjs.com/package/@wisdomstar94/torytis">
          <svg className="w-[20px] h-[20px] fill-red-500" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path d="M1.763 0C.786 0 0 .786 0 1.763v20.474C0 23.214.786 24 1.763 24h20.474c.977 0 1.763-.786 1.763-1.763V1.763C24 .786 23.214 0 22.237 0zM5.13 5.323l13.837.019-.009 13.836h-3.464l.01-10.382h-3.456L12.04 19.17H5.113z" />
          </svg>
        </Link>
        <Link className="p-1 cursor-pointer rounded-full hover:bg-slate-100" href="https://github.com/wisdomstar94/torytis">
          <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" className="w-[20px] h-[20px]">
            <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
          </svg>
        </Link>
      </div>
    </header>
  );
}
