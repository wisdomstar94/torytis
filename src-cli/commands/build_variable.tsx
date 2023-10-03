import { Command } from "commander";
import { isRepogitoryRoot } from "../functions/common";
import { renderToString } from 'react-dom/server';
import path from "path";
import fs from 'fs';
import { XMLParser } from 'fast-xml-parser';
const parser = new XMLParser();

// https://tistory.github.io/document-tistory-skin/common/variable.html#%EC%98%B5%EC%85%98-%EC%A0%95%EC%9D%98-indexxml
declare namespace IBuildVariable {
  export type VariableType = 'STRING' | 'SELECT' | 'IMAGE' | 'BOOL' | 'COLOR';

  interface Variable {
    name: string;
    label: string;
    description: string;
    type: VariableType;
  }

  interface VariableGroup {
    variable: Variable | Variable[];
  }

  interface Variables {
    variablegroup: VariableGroup | VariableGroup[];
  }
}

export function CommandBuildVariable(program: Command) {
  program
    .command('build:variable')
    .description('src/public/index.xml 파일을 파싱하여 variables 에 있는 데이터들을 typescript declare 파일로 변환합니다.')
    .action(async(str, options) => {
      if (!isRepogitoryRoot()) {
        console.error('package.json 이 존재하는 경로에서만 사용 가능합니다.');    
        return;
      }

      const args = options.args;
      const repositoryRootPath = process.cwd();

      // 1) src/public/index.xml 파일 읽기
      const indexXmlFilePath = path.join(repositoryRootPath, 'src', 'public', 'index.xml');
      const xmlString = fs.readFileSync(indexXmlFilePath);

      // 2) xml parsing 하기
      const json = parser.parse(xmlString);
      const variables: IBuildVariable.Variables = json.skin.variables;
      const variableGroupArray = Array.isArray(variables.variablegroup) ? variables.variablegroup : [variables.variablegroup];

      let variableList: IBuildVariable.Variable[] = [];
      for (const variableGroup of variableGroupArray) {
        const variableArray = Array.isArray(variableGroup.variable) ? variableGroup.variable : [variableGroup.variable];
        variableList = variableList.concat(variableArray);
      } 

      // 3) variableList 를 토대로 declare 파일 작성하기
      const torytisVariableDTsFilePath = path.join(repositoryRootPath, 'torytis-variable.d.ts');
      let declareCodeString = '';
      for (const variable of variableList) {
        const declareCode = generateJsxIntrinsicElementsDeclareCode(variable);
        declareCodeString += declareCode;
      }
      let torytisVariableDTsFileContent = '/// <reference types="react" />\n\n';
      torytisVariableDTsFileContent += `declare namespace JSX {\n`;
      torytisVariableDTsFileContent += `  interface IntrinsicElements {\n`;
      torytisVariableDTsFileContent += declareCodeString;
      torytisVariableDTsFileContent += `  }\n`;
      torytisVariableDTsFileContent += `}\n`;
      fs.writeFileSync(torytisVariableDTsFilePath, torytisVariableDTsFileContent);
    })  
  ;
}

function generateJsxIntrinsicElementsDeclareCode(variable: IBuildVariable.Variable) {
  let string = ``;
  string += `    's_if_var_${variable.name}': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;\n`;
  string += `    's_not_var_${variable.name}': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;\n`;
  return string;
}
