mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // javascript alert function binding
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



// Export a `greet` function from Rust to JavaScript, that alerts a hello message.
// javascript에서 greet() 호출하면 alert() 호출
#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, console-test!");
    alert("반갑습니다");
    // log 호출
    log("Hello from Rust!");
}
