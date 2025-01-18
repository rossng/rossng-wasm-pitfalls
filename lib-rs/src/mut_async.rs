use std::time::Duration;

use async_std::task::sleep;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Bar;

#[wasm_bindgen]
impl Bar {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Bar {
        Bar
    }
}

#[wasm_bindgen(js_name = "useMutBar")]
pub async fn use_mut_bar(_bar: &mut Bar) -> () {
    console::log_1(&"Rust: use_mut_bar".into());

    sleep(Duration::from_secs(1)).await;

    console::log_1(&"Rust: use_mut_bar done".into());
}

#[wasm_bindgen(js_name = "useBar")]
pub async fn use_bar(_bar: &Bar) -> () {
    console::log_1(&"Rust: use_bar".into());
}
