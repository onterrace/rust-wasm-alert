
# JavaScript 함수 호출 

이 프로젝트에서는 console.log(), alert()를 호출하는 방법과 자바스크립트에서 rust 함수를 호출하는 방법을 알아본다. 참고한 샘플은 [여기](https://rustwasm.github.io/wasm-bindgen/exbuild/hello_world/)서 확인할 수 있다. 

**목표**
* console.log() 호출, alert() 호출
* 자바스크립트에서 rust 함수 호출


use를 사용하여 wasm_bindgen::prelude 모듈을 불러온다. Rust에서 라이브러리는 크레이트(crate)라고 한다.Cargo(화물) 는 배에 crate(상자) 들을 실어 나른다.  use 키워드는 라이브러리로부터 코드를 불러온다.  이 경우, 우리는 wasm_bindgen::prelude 모듈 내에 있는 모든 것을  불러오게 된다.  

**src/lib.rs**
```rust
use wasm_bindgen::prelude::*;
```

## 자바스크립트 함수 호출
'#[ ]' 안에 있는 것을 속성이라고 부르는데, 이것은 다음에 오는 구문을 수정한다.  이 경우에, 그 구문은 extern이며, Rust에게 외부에 정의된 함수를 호출할 것임을 알린다.  이 속성의 경우, "wasm-bindgen은  이 함수들을 어떻게 찾을 것인지 알고 있다"고 알리는 것이다. 

**src/lib.rs**
```rust
#[wasm_bindgen]
extern "C" {
    // javascript alert function binding
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
```

위 코드에서 alert()와 log()를 정의했다.  alert()는 자바스크립트의 alert() 함수를 호출하고, log()는 자바스크립트의 console.log() 함수를 호출한다.  이 함수들은 외부에 정의되어 있으므로, extern "C" 블록 안에 정의한다.  이 블록은 wasm_bindgen에게 이 함수들이 외부에 정의되어 있다는 것을 알려준다.  이 블록은 #[wasm_bindgen] 속성을 가지고 있으며, 이 속성은 wasm_bindgen에게 이 함수들을 어떻게 찾을 것인지 알려준다. 

JavaScript에서 Rust 함수를 호출하기 위해서는 #[wasm_bindgen] 속성을 사용한다.  이 속성은 Rust에게 이 함수가 JavaScript에서 호출될 수 있음을 알려준다.  이 함수는 greet()이며, 이 함수는 alert()와 log()를 호출한다.




## Rust 함수 호출
**src/lib.rs**
```rust
// Export a `greet` function from Rust to JavaScript, that alerts a hello message.
// javascript에서 greet() 호출하면 alert() 호출
#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, console-test!");
    alert("반갑습니다");
    // log 호출
    log("Hello from Rust!");
}
```

index.js에서는 다음과 같이 import 한다. wasm 객체에는 greet() 함수가 포함되어 있다.

**www/index.js**
```jsx
import * as wasm from "console-test";

wasm.greet();
```

실행하면 다음과 같이 alert 창이 표시되고 콘솔에 로그를 출력한다. 









## 디폴트 프로젝트 생성 및 빌드 
wasm-pack를 이용하여 프로젝트를 생성하고 테스트한다. wasm-pack 설치 등은 설명하지 않는다. 다른 문서를 참고한다.  프로젝트를 생성한다. 

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





