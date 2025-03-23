import "./index.css";
import "./index.scss";
import { HelloWorld } from "@/components/hello-world/hello-world.component";
import { Script } from "@/components/script/script.component";
import { Style } from "./components/style/style.component";

export default function App() {
  return (
    <html lang="ko">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no" />
        <title></title>
      </head>
      <body id="[##_body_id_##]">
        <HelloWorld />
        <Script
          html={`
            (function(){
                const pathname = location.pathname;
                console.log('pathname', pathname);
            })();
        `}
        />
        <Style
          html={`
            body {
              background-color: #eee;
            }
          `}
        />
      </body>
    </html>
  );
}
