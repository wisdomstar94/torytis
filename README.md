<div align="center">

  <a href="https://github.com/wisdomstar94/torytis" target="_blank">
    <img width="180" src="https://cdn.jsdelivr.net/gh/wisdomstar94/torytis/src/public/torytis-logo.png" alt="torytis logo" />
  </a>

</div>

<br/>

<div align="center">

  <a href="https://github.com/wisdomstar94/torytis/blob/main/LICENSE" target="_blank">![GitHub](https://img.shields.io/github/license/wisdomstar94/torytis?logo=square)</a>
  <a href="https://www.npmjs.com/package/@wisdomstar94/torytis" target="_blank">![npm](https://img.shields.io/npm/v/%40wisdomstar94%2Ftorytis?logo=npm)</a>
  <a href="https://github.com/wisdomstar94/torytis" target="_blank">![GitHub release (with filter)](https://img.shields.io/github/v/release/wisdomstar94/torytis?logo=github&label=github)</a>

</div>

<br/>

# @wisdomstar94/torytis

@wisdomstar94/torytis 는 티스토리 블로그 스킨의 개발을 여러 파일로 나눠서 개발하는 방식으로 할 수 있도록 도와주는 빌드툴이자 프레임워크 입니다.

- 화면: html 코드들을 React 로 개발할 수 있도록 관련 환경을 제공합니다.
- 스타일: 기본적으로 Tailwindcss 와 scss 를 바로 사용 가능하도록 해주는 환경을 제공합니다.
- 타입스크립트 : 디폴트로 타입스크립트로 개발할 수 있는 환경을 제공합니다.

<br />

## 개발 이유

티스토리 스킨을 개발 할 때 아래 사항들이 불편하게 다가왔습니다. 
- 기존 spa 라이브러리나 프레임워크처럼 컴포넌트화 기능이 기본으로 제공되지 않아 개발하기 불편한 점
- html, css, js 각각의 코드 양이 많아질 수록 유지보수가 급격히 힘들어지는 점

이러한 불편한 점들을 해소하기 위해 직접 티스토리 스킨 개발만을 위한 기능이 준비되어 있는 빌드툴겸 프레임워크를 만들어보고자 생각하게 되었습니다.

<br />

## 구조 컨셉

- React 는 단순히 html 코드와 css 코드를 컴포넌트별로 분리하고, 이를 하나의 html 파일과 css 파일로 묶기 위한 용도로만 사용됩니다. 즉, 여기에서 말하는 컴포넌트란 캡슐화가 지원되지 않고 단순히 관련 코드들을 근처에 모아두는 역할만 하는 것입니다. (단, css 는 ``*.module.css`` 와 같이 파일명을 작성하고 ``import styles from "./*.module.scss"``) 와 같이 import 하여 ``styles['클래스명']`` 으로 사용할 경우 캡슐화가 지원됩니다.
- typescript(javascript) 도 각각의 별도 파일로 분리할 수 있습니다. 하지만 결국 빌드 될 때는 하나의 script.js 파일로 번들링됩니다. 즉, 분리된 js 끼리는 캡슐화가 적용되지 않으니 이 점을 유의하여 개발해야 합니다.

<br />

## 프로젝트 생성 방법
torytis 를 사용하기 위해 필요한 폴더구조 및 파일을 편리하게 셋팅할 수 있도록 도와주는 별도 [@wisdomstar94/torytis-create-app](https://github.com/wisdomstar94/torytis-create-app) 패키지가 준비되어 있습니다. 아래 명령어를 이용하여 프로젝트 생성 가능합니다.

```
npx @wisdomstar94/torytis-create-app new [프로젝트명]
```

<br />

## 이용 가이드

### 파일 네이밍 규칙
- 컴포넌트 : *.component.tsx 와 같은 패턴으로 작성합니다. ``(ex. guestbook.component.tsx)``
- 스타일 : *.scss 또는 *.module.scss 와 같은 패턴으로 작성합니다. ``(ex. guestbook.module.scss)``
- 스크립트 : *.script.tsx 와 같은 패턴으로 작성합니다. ``(ex. guestbook.script.tsx)``

### 파일 및 폴더 설명
- src/public/ : public 폴더 밑에 있는 파일들은 모두 빌드되어 .torytis 폴더 밑에 생성될 때 이 폴더 밑에 그대로 복사되어 집니다. public 폴더 밑에 또 다른 폴더를 생성하지 마세요.
- src/public/index.xml : 티스토리에서 요구하는 스킨에 대한 정의 값들을 선언하는 xml 파일입니다. 해당 경로에 위치시키시면 됩니다.
- src/index.component.tsx : html 코드를 하나로 묶을 때 진입점이 되는 파일입니다.
- tailwind.config.ts : tailwindcss 환경설정 파일입니다.
- postcss.config.js : postcss 환경설정 파일입니다.
- torytis-env.d.ts : torytis 관련 인터페이스를 참조하는 파일입니다.
- torytis-variable.d.ts : src/public/index.xml 파일에 정의한 variables 를 파싱하여 typescript 용 declare 코드가 작성되어 지는 파일입니다. ``npm run build:variable`` 명령어를 이용해 생성 및 덮어쓰기 가능합니다. (이 파일에 어떠한 코드를 직접 작성하였다면 그 코드는 유실될 수 있습니다.)

### 티스토리 치환자 대응
티스토리에서 요구하는 치환자 중에서도 일부 치환자를 그대로 리액트 컴포넌트 내에 작성할 경우 문법 에러가 발생합니다. 이를 해결하기 위해 torytis 만의 별도 문법을 준비하였습니다.
- 주석문 : ``<!--카테고리-->`` 을 작성해야 할때는 ``<tt_html_comment>카테고리</tt_html_comment>`` 와 같이 작성하면 빌드 될 때 html 주석문으로 치환됩니다.
- 속성치환 : ``<a [##_prev_page_##]>◀ PREV </a>`` 와 같이 작성해야 할때는 ``<a tt-onlyattr="[##_prev_page_##]">◀ PREV </a>`` 와 같이 작성하면 빌드 될 때 치환됩니다.
- onkeypress 치환 : ``<input onkeydown="if (event.keyCode == 13)[##_article_dissolve_##]" />`` 와 같이 작성해야 할 때는 ``<input tt-onkeydown="if (event.keyCode == 13)[##_article_dissolve_##]" />`` 와 같이 작성하면 빌드 될 때 치환됩니다.
- onclick 치환 : onkeypress 치환에서 설명한 패턴과 동일합니다.
- onkeydown 치환 : onkeypress 치환에서 설명한 패턴과 동일합니다.
- onload 치환 : onkeypress 치환에서 설명한 패턴과 동일합니다.
- onerror 치환 : onkeypress 치환에서 설명한 패턴과 동일합니다.
- value 치환 : onkeypress 치환에서 설명한 패턴과 동일합니다.

### 최종 결과물 빌드 방법
```
npm run build
```
위 명령어를 실행하면, ``.torytis/`` 폴더 밑에 index.xml, script.js, skin.html ... 등등의 파일들이 생성됩니다. ``.torytis/`` 폴더 밑에 생성된 파일들을 그대로 티스토리 스킨 파일로 첨부하시고 스킨을 등록하실 수 있습니다.

### 상황별 예시 코드
- *.module.scss 를 사용하는 경우

``src/components/my-compomnent/my-compomnent.component.tsx``
```
import styles from './my-component.module.scss';

export function MyComponent() {
  return (
    <div className={styles['my-class']}>
      Hello World!
    </div>
  );
}
```

``src/components/my-compomnent/my-compomnent.module.scss``
```
.my-class {
  font-size: 12px;
}
```

<br />

- *.scss 를 사용하는 경우

``src/components/my-compomnent/my-compomnent.component.tsx``
```
import './my-component.scss';

export function MyComponent() {
  return (
    <div className='my-class'>
      Hello World!
    </div>
  );
}
```

``src/components/my-compomnent/my-compomnent.scss``
```
.my-class {
  font-size: 12px;
}
```

<br />

- src/public/icon.png 파일을 html 코드에서 src 로 참조해야 하는 경우
```
export function MyComponent() {
  return (
    <div>
      <img src="./images/icon.png" />
    </div>
  );
}
```

<br /> 

- guestbook 컴포넌트를 생성하여 index.component.tsx 에 사용하는 경우

``src/components/guestbook/guestbook.component.tsx``
```
export function Guestbook() {
  return (
    <div>
      This is guestbook component!
    </div>
  );
}
```

``src/index.component.tsx``
```
import React from "react";
import './index.scss';
import { Guestbook } from './components/guestbook/guestbook.component';

export default function App() {
  return (
    <html lang="ko">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scale=1.0, user-scalable=no" />
        <title>...</title>
      </head>
      <body>
        <Guestbook />
      </body>
    </html>
  );
}
```

<br />

## 라이선스
@wisdomstar94/torytis 는 [MIT 라이선스](./LICENSE)가 적용됩니다.
