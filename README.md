# javascript console.log() 테스트 

wasm-pack를 이용하여 프로젝트를 생성하고 테스트한다. wasm-pack 설치 등은 설명하지 않는다. 다른 문서를 참고한다. 


## 디폴트 프로젝트 생성 및 빌드 
프로젝트를 생성한다. 


```shell
wasm-pack new consle-test
```

생성한 프로젝트로 들어간다. 

```shell
cd console-test
```

생성된 샘플을 빌드한다. 

```shell
wasm-pack build
```


브라우저 테스트 환경을 만들기 위해 wasm-app를 이용하여 테스트 디렉토리를 생성한다.
```shell
npm init wasm-app www
```
위 명령은 한 번만 실행하면 된다. 


생성한 www 디렉토리로 들어간다. 
```shell
cd www
```

wasm은 pkg 디렉토리에 생성된다. package.json 수정한다.   "hello-wasm-pack": "^0.1.0" 을  "console-test": "file:../pkg"로 수정한다. 

```json 
  "devDependencies": {
    "console-test": "file:../pkg",
    "webpack": "^4.29.3",
    "webpack-cli": "^3.1.0",
    "webpack-dev-server": "^3.1.5",
    "copy-webpack-plugin": "^5.0.0"
  }
```

index.js를 다음과 같이 수정한다. 

```jsx
import * as wasm from "console-test";
```

wasm 패키지를 설치한다. 

```shell
npm install
```
다음을 실행하여 테스트용 서버를 실행한다. 

```shell
npm run start
```
브라우저에서 http://localhost:8080 접속하여 확인한다. 




