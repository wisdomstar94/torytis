import { Style } from "../style/style.component";
import styles from "./hello-world.module.scss";
import { cn } from "@/utils/cn";

export function HelloWorld() {
  return (
    <>
      <div
        id="hello-world"
        className={cn(styles["text"], "pt-1 pb-3 border border-slate-600 bg-white", "cursor-pointer hover:bg-slate-200")}
      >
        hello world!!!
      </div>
      <s_if_var_dark_mode_type>
        <Style
          html={`
            html, body {
              background-color: #000;
              color: #fff;
            }  
          `}
        />
      </s_if_var_dark_mode_type>
    </>
  );
}
