// 이 파일을 삭제하거나 수정할 경우 빌드가 되지 않을 수 있습니다.
const esbuild = require('esbuild');
const { sassPlugin, postcssModules } = require('esbuild-sass-plugin');
const { renderToString } = require('react-dom/server');
const path = require("path");
const fs = require('fs');
const jsx_runtime = require("react/jsx-runtime");

(async() => {
    const repositoryRootPath = process.cwd();
    const convertIndexJsxPath = path.join(repositoryRootPath, '.torytis', 'index.js');
    const skinHtmlPath = path.join(repositoryRootPath, '.torytis', 'skin.html');

    function disposeIndexComponentTsx() {
        return new Promise(async(resolve, reject) => {
            await esbuild.build({
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
            });
        
            const indexJsx = await import(convertIndexJsxPath);
            const App = indexJsx.default.default; 
            const html = renderToString(jsx_runtime.jsx(App, {}));
            fs.writeFileSync(skinHtmlPath, html);
            fs.rmSync(convertIndexJsxPath);

            resolve(true);
        });  
    }

    function disposeScriptTs() {
        return new Promise(async(resolve, reject) => {
            const scriptTsFilePath = path.join(repositoryRootPath, '.torytis', 'script.ts');
            const scriptJsFilePath = path.join(repositoryRootPath, '.torytis', 'script.js');
            await esbuild.build({
                entryPoints: [scriptTsFilePath],
                bundle: true,
                jsx: 'automatic',
                target: ['es6'],
                treeShaking: true,
                platform: 'browser',
                format: 'cjs',
                outfile: scriptJsFilePath,
            });
            resolve(true);
        });
    }

    await Promise.allSettled([
        disposeIndexComponentTsx(),
        disposeScriptTs(),
    ]);
})();
