pub mod losing_ownership;
pub mod mut_async;
pub mod vec_parameter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
