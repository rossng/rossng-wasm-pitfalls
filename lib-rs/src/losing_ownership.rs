use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Foo;

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Foo {
        Foo
    }
}

#[wasm_bindgen(js_name = "borrowFoo")]
pub fn borrow_foo(_foo: &Foo) {
    console::log_1(&"Rust: borrowed a Foo".into());
}

#[wasm_bindgen(js_name = "consumeFoo")]
pub fn consume_foo(_foo: Foo) {
    console::log_1(&"Rust: consumed a Foo".into());
}
