import "./index.scss";
import { HelloWorld } from "@/components/hello-world/hello-world.component";
import { Script } from "@/components/script/script.component";

export default function App() {
  return (
    <html lang="ko">
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scale=1.0, user-scalable=no"
        />
        <title></title>
      </head>
      <body>
        <HelloWorld />
        <Script
          html={`
            (function(){
                const pathname = location.pathname;
                console.log('pathname', pathname);
            })();
        `}
        />
      </body>
    </html>
  );
}
