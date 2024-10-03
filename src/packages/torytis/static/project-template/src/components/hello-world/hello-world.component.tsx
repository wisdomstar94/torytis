import styles from "./hello-world.module.scss";
import { cn } from "@/utils/cn";

export function HelloWorld() {
  return (
    <div id="hello-world" className={cn(styles["text"], "pt-1 pb-3 p-2")}>
      hello world!!!
    </div>
  );
}
