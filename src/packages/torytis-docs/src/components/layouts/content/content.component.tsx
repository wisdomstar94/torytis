import { cn } from "@/utils/common";
import { IContent } from "./content.type";

export function Content(props: IContent.Props) {
  const { children } = props;

  return (
    <main
      className={cn(
        "markdown-body",
        "transition-all duration-300",
        // "pl-sidebar-width sidebar-collapsed:pl-0",
        "pt-topbar-height",
        "lg:pl-sidebar-width lg:sidebar-toggle-yes:pl-0",

        "relative"
        //
      )}
    >
      <div className="w-full box-border p-5 relative">{children}</div>
    </main>
  );
}
