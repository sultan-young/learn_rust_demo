use wasm_bindgen::prelude::*;
use web_sys::console;

// 从root目录开始计算，导入一个js
#[wasm_bindgen(module = "/www/util.js")]
extern "C" {
    fn jsFn() -> String;

    // 从js中引入一个class类
    type JsClass;
    #[wasm_bindgen(constructor)]
    fn new() -> JsClass;    
    #[wasm_bindgen(method)]
    fn getRandomNumber(this: &JsClass) -> f32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"123123".into());
    log(&jsFn());

    let x = JsClass::new();
    log(&x.getRandomNumber().to_string());
}