## 1.3.3

- `torytis dev` 웹 서버에서 `style.css` 파일에 작성된 `/images/` 으로 시작하는 경로도 `/virtualcdn/images/` 으로 치환되도록 수정.
- `torytis-dev.config.json` 파일에 블로그 프로필 이미지에 해당하는 값인 `blog_profile_img_url` 파라미터 추가.
- `torytis new` 템플릿 프로젝트에 `Script` 컴포넌트 추가. (html 코드와 함께 script 태그 작성이 필요할 경우 사용.)

## 1.3.2

- `torytis new` 명령어로 생성되는 템플릿 프로젝트에 `src/utils/cn.ts` 유틸 함수 추가.

## 1.3.1

- `torytis build --flat=false` 명령어를 입력할 경우 에러가 발생하는 부분 수정.

## 1.3.0

- esbuild 를 사용하던 부분을 vite 로 교체
- `torytis-variable-object.ts` 파일 위치를 `src/consts/` 폴더 밑으로 이동
- `torytis-variable.d.ts`, `torytis-env.d.ts` 타입 선언 파일들의 위치를 `src/types/` 폴더 밑으로 이동
- `torytis new` 명령어로 생성되는 템플릿 프로젝트 내의 파일 구조 일부 수정 및 package.json 에 기재된 일부 종속성 제거 및 추가와 동시에 기존 종속성 패키지들의 버전 최신화
- torytis cargo 종속성 패키지 버전 최신화
- torytis migrate 명령어 삭제 (당분간 지원 중지)
  - 대체안 : torytis 레포지토리의 `static/project-template` 경로에 있는 파일 및 폴더들의 변동 사항을 참조
- 위 변동 사항으로 인한 `README.md` 문서 내용 수정

## 1.2.2

- cargo 및 npm 종속성 패키지 버전 최신화

## 1.2.1

- 하나로 번들된 script.js 내용들이 최상위 함수 안에서 호출되도록 수정되었습니다.
  - 이로 인해 더이상 src/\*_/_.script.tsx 에 작성했던 함수들은 window 전역 객체 밑으로 들어가지 않습니다. 전역 함수로 만들기 위해선 함수를 선언한 뒤 window.함수명 = 함수명 과 같이 수동으로 넣어주어야 합니다.
  - 번들 된 결과물에 있는 전역 함수명과 외부 js script 에 의해 주입된 전역 함수명이 동일한 경우를 대비하기 위해 이와 같이 수정되었습니다.

## 1.2.0

- cargo 종속성 패키지 버전 최신화 및 그로 인한 deprecatd 된 코드 수정

## 1.1.1

- torytis-dev.config.json 파일에서 "posts" -> "contents" -> "type" 에 "h1", "h5", "h6" 타입 추가
