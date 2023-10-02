import { Command } from "commander";
import { isRepogitoryRoot } from "../functions/common";
import { renderToString } from 'react-dom/server';
import path from "path";
import fs from 'fs';
import { sassPlugin, postcssModules } from 'esbuild-sass-plugin';
import esbuild from 'esbuild';
import { globSync } from 'glob';
import { exec } from 'child_process';
import { allInOneReplace } from "../functions/replacer";

export function CommandBuild(program: Command) {
  program
    .command('build')
    .description('tistory skin 업로드 규격에 맞게 최종 결과물로 빌드합니다.')
    .action(async(str, options) => {
      if (!isRepogitoryRoot()) {
        console.error('package.json 이 존재하는 경로에서만 사용 가능합니다.');    
        return;
      }

      const args = options.args;
      const repositoryRootPath = process.cwd();

      // 1) .torytis 폴더 생성하기
      if (!fs.existsSync(path.join(repositoryRootPath, '.torytis/'))) {
        fs.mkdirSync(path.join(repositoryRootPath, '.torytis/'));
      }

      // 2) src 밑에 있는 index.component.tsx 파일을 index.js 파일로 변환하기
      const convertIndexJsxPath = path.join(repositoryRootPath, '.torytis', 'index.js');
      await new Promise(function(resolve, reject) {
        esbuild.build({
          entryPoints: [path.join(repositoryRootPath, 'src', 'index.component.tsx')],
          bundle: true,
          jsx: 'automatic',
          target: ['es6'],
          treeShaking: true,
          platform: 'browser',
          format: 'cjs',
          outfile: convertIndexJsxPath,
          plugins: [
            sassPlugin({
              filter: /\.module\.scss$/,
              transform: postcssModules({}),
            }),
            sassPlugin({
              filter: /\.scss$/,
            }),
          ],
        }).then(res => {
          resolve(res);
        }).catch(err => {
          reject(err);
        });
      });

      // 3) jsx 를 html 으로 변환하기
      const indexJsx = await import(convertIndexJsxPath);
      const App = indexJsx.default.default; 
      const html = renderToString(<App />);

      // 4) 이제 불필요한 .torytis/index.js 파일 제거하기
      fs.rmSync(convertIndexJsxPath);

      // 5) src 밑에 있는 .ts 파일 찾기
      const targetTsDir = path.join(repositoryRootPath, 'src', `**/*.script.tsx`);
      const tsPathes = globSync(targetTsDir, {
        dot: true,
        // node_modules 은 검색대상에서 제외
        ignore: ['node_modules/**']
      });
      
      const relativeTsPathes = tsPathes.map(k => {
        const replaceTarget = path.join(repositoryRootPath, 'src/');
        return k.replace(replaceTarget, '');
      });
      const scriptTsString = relativeTsPathes.map(k => `import '../src/${k}'`).join(';\n');

      // 6) .torytis/script.ts 파일 만들기
      const scriptTsFilePath = path.join(repositoryRootPath, '.torytis', 'script.ts');
      fs.writeFileSync(scriptTsFilePath, scriptTsString);

      // 7) .torytis/script.ts 파일을 .torytis/script.js 파일로 번들링 하기
      const scriptJsFilePath = path.join(repositoryRootPath, '.torytis', 'script.js');
      await new Promise(function(resolve, reject) {
        esbuild.build({
          entryPoints: [scriptTsFilePath],
          bundle: true,
          jsx: 'automatic',
          target: ['es6'],
          treeShaking: true,
          platform: 'browser',
          format: 'cjs',
          outfile: scriptJsFilePath,
        }).then(res => {
          resolve(res);
        }).catch(err => {
          reject(err);
        });
      });

      // 8) .torytis/script.js 파일에 있는 torytis html 문법 치환하기
      let torytisScriptJsString = fs.readFileSync(scriptJsFilePath).toString();
      torytisScriptJsString = allInOneReplace(torytisScriptJsString);
      fs.writeFileSync(scriptJsFilePath, torytisScriptJsString);

      // 9) .torytis/script.ts 파일 제거하기
      fs.rmSync(scriptTsFilePath);

      // 10) skin.html 파일 생성하기
      let convertedHtml = html.replace(`</head>`, `<link href="./style.css" type="text/css" rel="stylesheet" /></head>`);
      convertedHtml = convertedHtml.replace(`</head>`, `<script src="./images/script.js"></script></head>`);
      convertedHtml = allInOneReplace(convertedHtml);
      fs.writeFileSync(path.join(repositoryRootPath, '.torytis', 'skin.html'), convertedHtml);

      // 11) tailwind 로 변환하기
      const torytisIndexCssPath = path.join(repositoryRootPath, '.torytis', 'index.css');
      await new Promise(function(resolve, reject) {
        exec(`npx tailwindcss -i ${torytisIndexCssPath} -o ${torytisIndexCssPath}`, (error, stdout, stderr) => {
          resolve(true);
        });
      }); 

      // 12) .torytis/index.css 파일명을 .torytis/style.css 으로 변경하기
      const torytisStyleCssPath = path.join(repositoryRootPath, '.torytis', 'style.css');
      fs.renameSync(torytisIndexCssPath, torytisStyleCssPath);

      // 13) src/public 밑에 있는 파일들 .torytis/ 밑으로 복사하기
      const srcPublicFolderPath = path.join(repositoryRootPath, 'src', 'public/');
      if (fs.existsSync(srcPublicFolderPath)) {
        fs.cpSync(srcPublicFolderPath, path.join(repositoryRootPath, '.torytis/'), { recursive: true });
      }
    })  
  ;
}