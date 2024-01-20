// 이 파일을 삭제하거나 수정할 경우 빌드가 되지 않을 수 있습니다.
import esbuild from 'esbuild';
import { sassPlugin, postcssModules } from 'esbuild-sass-plugin';
import { renderToString } from 'react-dom/server';
import path from "path";
import fs from 'fs';

(async() => {
    const repositoryRootPath = process.cwd();
    const convertIndexJsxPath = path.join(repositoryRootPath, '.torytis', 'index.js');
    const skinHtmlPath = path.join(repositoryRootPath, '.torytis', 'skin.html');

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
    const App = indexJsx.default; 
    const html = renderToString(<App />);
    fs.writeFileSync(skinHtmlPath, html);
    fs.rmSync(convertIndexJsxPath);

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
})();
