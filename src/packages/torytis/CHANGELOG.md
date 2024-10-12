## v1.5.3

- `dev server` 구동시 필요한 작업에 대한 api 구현. `watch.mjs` 에서 `fetch` 를 이용해 호출하는 구조.
- `socket` 연결 port 를 `3020` 으로 수정.
- `torytis new` 명령어에서 내려받는 `torytis-start-template` 의 버전을 `v0.0.7` 으로 변경.

## v1.5.2

- `torytis` 를 이용하기 위해선 node.js 버전이 `20.10.0` 이상이어야 함을 명시.
- `torytis new` 명령어에서 내려받는 `torytis-start-template` 의 버전을 `v0.0.6` 으로 변경.
- `torytis dev` 명령어에서 node 로 `watch.mjs` 을 실행하는 부분을 제거하고 이는 torytis 프로젝트에서 `concurrently` 를 활용해 실행하는 것으로 변경.

## v1.5.1

- 특정 파일 및 폴더의 경로를 선언 및 불러오는 부분 정리.
- `torytis new` 명령어에서 내려받는 `torytis-start-template` 의 버전을 `v0.0.5` 으로 변경.

## v1.5.0

- `npm run dev` 에 핫리로드를 적용하였습니다. dev 서버를 실행한 상태에서 소스코드 변경시 자동으로 빌드되며 브라우저에서 새로고침 됩니다. 이를 위해 torytis 에서 아래와 같은 커맨드들이 추가되었습니다.
  - `buildpreprocess`: 빌드하기 전에 처리해야 하는 작업을 실행
  - `movepublictodottorytis`: `src/public/` 폴더 밑에 있는 파일들을 `.torytis/` 폴더 밑으로 복사
  - `scriptbundle`: `src/**/*.script.tsx` 에 해당하는 파일들을 `.torytis/script.ts` 파일 하나로 묶음
  - `scriptpostprocess`: `.torytis/script.js` 파일에 후처리 진행
  - `skinhtmlreplace`: `.torytis/skin.html` 파일 내용에 치환 적용

## v1.4.4

- `npm run dev` 변경 사항
  - 검색 관련 치환자 처리가 되지 않던 현상 수정.
  - 각 페이지별 `[##_page_title_##]` 치환자 처리 추가.

## v1.4.3

- `npm run dev` 에서 `s_guest`, `s_rp` 치환 시 실제 티스토리 치환 결과대로 작동하도록 수정.
- `npm run dev` 에서 `comment.css` 스타일이 로드 되도록 수정. (Namecard 부분)

## v1.4.2

- `npm run dev` 에서 `s_sidebar` 치환 시 실제 티스토리 치환 결과대로 작동하도록 수정.

## v1.4.1

- 다운받을 시작 템플릿 버전을 `0.0.1` 에서 `0.0.2` 으로 변경.
- 시작 템플릿 압축 해제 후 `README.md`, `CHANGELOG.md` 파일은 삭제되도록 처리.
- `npm run dev` 에서 html 치환해서 응답 내려줄 때, ./style.css 경로로 응답되던 부분을 /virtualcdn/style.css 으로 수정.

## v1.4.0

- 패키지 매니저를 `npm` 에서 `yarn` 으로 변경.
- 프로젝트를 모노레포 구조로 변경
- `toytis new` 명령어를 통해 생성되는 프로젝트 폴더내의 파일들을 torytis 내부에 포함시키지 않고 github 에 등록된 파일들을 내려 받는 구조로 변경.
  - 이제 `src/packages/torytis-start-template` 밑의 파일들을 내려 받음.
  - 템플릿을 다운로드 할 때 템플릿명과 버전을 터미널에 표시.
- 아래와 같은 워크스페이스 패키지를 추가.
  - `consts` : 상수들을 모아 놓는 패키지.
  - `downloader` : 다운로드와 관련된 기능을 제공하는 패키지.
  - `file_manager` : 파일 핸들링과 관련된 기능을 제공하는 패키지.
  - `flater` : 압축 파일 핸들링과 관련된 기능을 제공하는 패키지.

## v1.3.13

- post permalink 에 대해서도 `[##_article_rep_date_##]`, `[##_article_rep_simple_date_##]` 치환자 처리 추가

## v1.3.12

- `[##_article_rep_date_##]`, `[##_article_rep_simple_date_##]` 치환자 처리 추가
- 시작 템플릿의 `src/utils/common.ts` 파일에 공용 유틸 함수 추가.

## v1.3.11

- `s_rctps_rep`, `s_rctps_rep_thumbnail` 태그에 대한 전역 타입 선언 추가
- `npm run dev` 의 html 치환자 처리시 `s_rctps_rep`, `s_rctps_rep_thumbnail` 에 대한 처리 추가
- `vite` 가 scss legacy api 를 사용하지 않도록 설정 값 수정 (마이그레이션 시 `static/project-template` 경로를 참고하세요.)

## v1.3.10

- `style.css` 에 preload 가 적용되도록 수정

## v1.3.9

- `s_list`, `s_list_rep`, `s_list_empty`, `s_page_rep` 태그에 대한 타입 추가

## v1.3.8

- `npm run build` 또는 `npm run dev` 시 node.js 단에서 발생한 에러도 터미널에서 표시되도록 수정. (torytis.build.mjs 파일 변경사항 확인 및 반영 필요.)
- `s_notice_rep_thumbnail` 치환자 처리 추가
- `s_rp2_rep` 치환자 처리 추가
- `[##_notice_rep_**_##]` 공지사항 관련 치환자 처리 추가

## v1.3.7

- `npm run dev` 에 대한 변경사항
  - `<s_rp_count>` 치환자 처리 추가
  - `[##_article_rep_rp_cnt_##]` 치환자 처리 추가
  - `[##_article_rep_author_##]` 치환자 처리 추가
  - `s_article_rep_thumbnail` 치환자 처리시 `thumbnail_img_url` 값이 빈 공백일 경우에는 빈 문자로 치환되도록 수정
  - `torytis-dev.config.json` 의 `Posts` 의 아이템 요소에 작성자에 해당하는 `author` 항목 추가

## v1.3.6

- 아래 치환자 추가
  - `tt-onmouseenter` -> `onmouseenter`
  - `tt-onmouseleave` -> `onmouseleave`
- template 프로젝트의 package.json 에 아래 스크립트 추가
  - `npm run type:check`

## v1.3.5

- 아래 치환자 추가
  - `tt-onmouseover` -> `onmouseover`
  - `tt-onmouseout` -> `onmouseout`

## v1.3.4

- `torytis dev` 웹 서버에서 `style.css` 파일에 작성된 `./images/` 으로 시작하는 경로도 `/virtualcdn/images/` 으로 치환되도록 수정.
- `torytis version` 명령어 추가. (torytis 의 현재 버전 확인 가능.)
- `README.md` 에 (s)css 파일에서 public 파일 경로 참조하는 방법에 대한 내용 추가.
- `README.md` 에 torytis 현재 버전 확인하는 방법에 대한 내용 추가.
- `README.md` 에 torytis 로 만들어진 tistory 스킨에 대한 내용 추가.

## v1.3.3

- `torytis dev` 웹 서버에서 `style.css` 파일에 작성된 `/images/` 으로 시작하는 경로도 `/virtualcdn/images/` 으로 치환되도록 수정.
- `torytis-dev.config.json` 파일에 블로그 프로필 이미지에 해당하는 값인 `blog_profile_img_url` 파라미터 추가.
- `torytis new` 템플릿 프로젝트에 `Script` 컴포넌트 추가. (html 코드와 함께 script 태그 작성이 필요할 경우 사용.)

## v1.3.2

- `torytis new` 명령어로 생성되는 템플릿 프로젝트에 `src/utils/cn.ts` 유틸 함수 추가.

## v1.3.1

- `torytis build --flat=false` 명령어를 입력할 경우 에러가 발생하는 부분 수정.

## v1.3.0

- esbuild 를 사용하던 부분을 vite 로 교체
- `torytis-variable-object.ts` 파일 위치를 `src/consts/` 폴더 밑으로 이동
- `torytis-variable.d.ts`, `torytis-env.d.ts` 타입 선언 파일들의 위치를 `src/types/` 폴더 밑으로 이동
- `torytis new` 명령어로 생성되는 템플릿 프로젝트 내의 파일 구조 일부 수정 및 package.json 에 기재된 일부 종속성 제거 및 추가와 동시에 기존 종속성 패키지들의 버전 최신화
- torytis cargo 종속성 패키지 버전 최신화
- torytis migrate 명령어 삭제 (당분간 지원 중지)
  - 대체안 : torytis 레포지토리의 `static/project-template` 경로에 있는 파일 및 폴더들의 변동 사항을 참조
- 위 변동 사항으로 인한 `README.md` 문서 내용 수정

## v1.2.2

- cargo 및 npm 종속성 패키지 버전 최신화

## v1.2.1

- 하나로 번들된 script.js 내용들이 최상위 함수 안에서 호출되도록 수정되었습니다.
  - 이로 인해 더이상 src/\*_/_.script.tsx 에 작성했던 함수들은 window 전역 객체 밑으로 들어가지 않습니다. 전역 함수로 만들기 위해선 함수를 선언한 뒤 window.함수명 = 함수명 과 같이 수동으로 넣어주어야 합니다.
  - 번들 된 결과물에 있는 전역 함수명과 외부 js script 에 의해 주입된 전역 함수명이 동일한 경우를 대비하기 위해 이와 같이 수정되었습니다.

## v1.2.0

- cargo 종속성 패키지 버전 최신화 및 그로 인한 deprecatd 된 코드 수정

## v1.1.1

- torytis-dev.config.json 파일에서 "posts" -> "contents" -> "type" 에 "h1", "h5", "h6" 타입 추가
