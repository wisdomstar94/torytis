## v0.0.8

- `package.json` 의 종속성 패키지들의 버전 최신화
- `package.json` 에 `npm-check-updates` 패키지 종속성 추가
- `package.json` 에 종속성 패키지의 버전을 최신화할 수 있는 스크립트인 `package:update` 추가.

## v0.0.7

- `npm run dev` 스크립트에서 `watch.mjs` 는 `dev server` 가 구동된 이후에 실행되도록 수정.
- `torytis` 의 종속성 최소 버전을 `1.5.3` 으로 수정.
- `socket` listen port 를 `3020` 으로 수정.

## v0.0.6

- windows os 에서 vite resolve alias 경로 문제 발생하던 현상 수정.

## v0.0.5

- `vite.**.config.ts` 파일들을 `config/` 폴더 밑으로 이동.
- `torytis.build.mjs` 파일을 `script/build.mjs` 으로 이동.

## v0.0.4

- `package.json` 에서 `torytis` 종속성 버전을 `1.5.0` 으로 수정.

## v0.0.3

- `npm run dev` 에서 소스코드 변경사항을 자동으로 반영하기 위해 `script/watch.mjs` 추가.

## v0.0.2

- `Style` 유틸용 컴포넌트 추가.
- `README.md` 추가.
- tab 여백 등 수정.
- viewport 옵션 중에 `user-scale=1.0` 제거.

## v0.0.1

- first release.
