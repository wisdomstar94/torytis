import { Command } from "commander";
import { isRepogitoryRoot } from "../functions/common";
import { renderToString } from 'react-dom/server';
import path from "path";
import fs from 'fs';
import { sassPlugin } from 'esbuild-sass-plugin';
import esbuild from 'esbuild';
import { globSync } from 'glob';
import { exec } from 'child_process';
import { metaTagReplace, ttAttrReplace, ttHtmlCommentReplace } from "../functions/replacer";

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
      const convertIndexJsxPath = path.join(repositoryRootPath, '.torytis', 'index.js');

      // 1) src 밑에 있는 index.tsx 파일을 index.js 파일로 변환하기
      // console.log(`1) src 밑에 있는 index.tsx 파일을 index.js 파일로 변환하기`);
      await new Promise(function(resolve, reject) {
        esbuild.build({
          entryPoints: [path.join(repositoryRootPath, 'src', 'index.tsx')],
          bundle: true,
          jsx: 'automatic',
          target: ['es5'],
          treeShaking: true,
          platform: 'browser',
          format: 'cjs',
          outfile: convertIndexJsxPath,
          plugins: [
            sassPlugin(),
            /*
            {
              async transform(source, resolveDir) {
                const {css} = await postcss([autoprefixer, postcssPresetEnv({stage: 0})]).process(source, {from: undefined})
                return css
              },
            }
             */
          ],
        }).then(res => {
          resolve(res);
        }).catch(err => {
          reject(err);
        });
      });

      // 2) .torytis 폴더 생성하기
      // console.log(`2) .torytis 폴더 생성하기`);
      if (!fs.existsSync(path.join(repositoryRootPath, '.torytis/'))) {
        fs.mkdirSync(path.join(repositoryRootPath, '.torytis/'));
      }

      // 3) jsx 를 html 으로 변환하기
      // console.log(`3) jsx 를 html 으로 변환하기`);
      const indexTsx = await import(convertIndexJsxPath);
      const App = indexTsx.default.default; 
      const html = renderToString(<App />);

      // 4) 이제 불필요한 .torytis/index.js 파일 제거하기
      // console.log(`4) 이제 불필요한 .torytis/index.js 파일 제거하기`);
      fs.rmSync(convertIndexJsxPath);

      // 5) src 밑에 있는 .ts 파일 찾기
      // console.log(`5) src 밑에 있는 .ts 파일 찾기`);
      const targetTsDir = path.join(repositoryRootPath, 'src', `**/*.ts`);
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
      // console.log(`6) src/script.ts 파일 만들기`);
      const scriptTsFilePath = path.join(repositoryRootPath, '.torytis', 'script.ts');
      fs.writeFileSync(scriptTsFilePath, scriptTsString);

      // 7) .torytis/script.ts 파일을 .torytis/images/script.js 파일로 번들링 하기
      // console.log(`7) .torytis/script.ts 파일을 .torytis/images/script.js 파일로 번들링 하기`);
      const scriptJsFilePath = path.join(repositoryRootPath, '.torytis', 'images', 'script.js');
      await new Promise(function(resolve, reject) {
        esbuild.build({
          entryPoints: [scriptTsFilePath],
          bundle: true,
          jsx: 'automatic',
          target: ['es5'],
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

      // 8) .torytis/script.ts 파일 제거하기
      // console.log(`8) .torytis/script.ts 파일 제거하기`);
      fs.rmSync(scriptTsFilePath);

      // 9) skin.html 파일 생성하기
      let convertedHtml = html.replace(`</head>`, `<link href="./index.css" type="text/css" rel="stylesheet" /></head>`);
      convertedHtml = convertedHtml.replace(`</head>`, `<script src="./images/script.js"></script></head>`);
      convertedHtml = ttAttrReplace(convertedHtml);
      convertedHtml = ttHtmlCommentReplace(convertedHtml);
      convertedHtml = metaTagReplace(convertedHtml);
      fs.writeFileSync(path.join(repositoryRootPath, '.torytis', 'skin.html'), convertedHtml);

      // 10) tailwind 로 변환하기
      const torytisIndexCssPath = path.join(repositoryRootPath, '.torytis', 'index.css');
      await new Promise(function(resolve, reject) {
        exec(`npx tailwindcss -i ${torytisIndexCssPath} -o ${torytisIndexCssPath}`, (error, stdout, stderr) => {
          resolve(true);
        });
      }); 

      // return;

      // // 10) src/index.scss 읽기
      // const srcIndexScssPath = path.join(repositoryRootPath, 'src', 'index.scss');
      // const srcIndexScss = fs.readFileSync(srcIndexScssPath).toString();
      // // console.log('@srcIndexScss', srcIndexScss);

      // // 11) src 및에 모든 .scss 파일 읽어오기
      // const targetScssDir = path.join(repositoryRootPath, 'src', `**/*.scss`);
      // const allScssFilePathes = globSync(targetScssDir, {
      //   dot: true,
      //   // node_modules 은 검색대상에서 제외
      //   ignore: ['node_modules/**'],
      // });
      // const allScssFileRelativePathes = allScssFilePathes.map(k => {
      //   const replaceTarget = path.join(repositoryRootPath, 'src/');
      //   return k.replace(replaceTarget, '');
      // }).filter(x => x !== 'index.scss');
      // // console.log('@allScssFileRelativePathes', allScssFileRelativePathes);
      // let convertedIndexScss = srcIndexScss;
      // allScssFileRelativePathes.forEach((scssPath) => {
      //   convertedIndexScss = convertedIndexScss.concat('\n@import \'../src/' + scssPath + '\';');
      // });
      // console.log('@convertedIndexScss', convertedIndexScss);

      // // 12) .torytis/index.scss 생성하기
      // const indexScssPath = path.join(repositoryRootPath, '.torytis', '_index.scss');
      // fs.writeFileSync(indexScssPath, convertedIndexScss);

      // // 13) src/_index.scss 을 .torytis/index.css 으로 번들링하기
      // await new Promise(function(resolve, reject) {
      //   esbuild.build({
      //     entryPoints: [indexScssPath],
      //     bundle: true,
      //     jsx: 'automatic',
      //     target: ['es5'],
      //     treeShaking: true,
      //     platform: 'browser',
      //     format: 'cjs',
      //     outfile: torytisIndexCssPath,
      //     plugins: [
      //       sassPlugin({
      //         async transform(source, resolveDir) {
      //           const {css} = await postcss([autoprefixer, postcssPresetEnv({stage: 0})]).process(source, {from: undefined});
      //           return css
      //         },
      //       }),
      //     ],
      //   }).then(res => {
      //     resolve(res);
      //   }).catch(err => {
      //     reject(err);
      //   });
      // });

      // // 14) .torytis/index.scss 삭제하기
      // fs.rmSync(indexScssPath);

      // // 15) tailwind 로 변환하기
      // await new Promise(function(resolve, reject) {
      //   exec(`npx tailwindcss -i ${torytisIndexCssPath} -o ${torytisIndexCssPath}`, (error, stdout, stderr) => {
      //     resolve(true);
      //   });
      // }); 
    })  
  ;
}