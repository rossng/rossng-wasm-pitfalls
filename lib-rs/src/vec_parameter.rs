use wasm_bindgen::{convert::RefFromWasmAbi, prelude::*};
use web_sys::{
    console,
    js_sys::{self, Reflect},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Baz;

#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Baz {
        Baz
    }

    #[wasm_bindgen]
    pub fn clone(&self) -> Self {
        Clone::clone(self)
    }
}

// Example 1

#[wasm_bindgen(js_name = "useJsVec")]
pub fn use_js_vec(vec: Vec<String>) {
    console::log_1(&format!("Rust: use_js_vec {:?}", vec).into());
}

// Example 2
// Slightly more complex case that exposes an issue: you can't pass the
// array into the function more than once.

#[wasm_bindgen(js_name = "useRsVec")]
pub fn use_rs_vec(vec: Vec<Baz>) {
    console::log_1(&format!("Rust: use_rs_vec {:?}", vec).into());
}

// Example 2.5: A non-solution
// Cannot be wasm-bindgen'ed because:

//      the trait bound `&Baz: JsObject` is not satisfied
//      required for `&Baz` to implement `VectorFromWasmAbi`
//      required for `Vec<&Baz>` to implement `FromWasmAbi`

// #[wasm_bindgen(js_name = "useRefVec")]
pub fn use_ref_vec(vec: Vec<&Baz>) {
    console::log_1(&format!("Rust: use_ref_vec {:?}", vec).into());
}

// Example 4: An unpleasant workaround with good JavaScript ergonomics

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<Baz>")]
    pub type ArrayBaz;
}

pub fn from_js_value<A: Clone + RefFromWasmAbi<Abi = u32>>(js: JsValue) -> A {
    let ptr = JsValue::from_str("__wbg_ptr");
    #[allow(unused_unsafe)]
    let ptr = unsafe { Reflect::get(&js, &ptr).unwrap() };
    let id = ptr.as_f64().unwrap() as u32;
    unsafe { A::ref_from_abi(id).clone() }
}

impl TryInto<Vec<Baz>> for ArrayBaz {
    type Error = JsValue;

    fn try_into(self) -> Result<Vec<Baz>, Self::Error> {
        Ok(js_sys::try_iter(&self)?
            .ok_or::<JsValue>(JsError::new("Iterator not iterable").into())?
            .map(|item| {
                let location = item.unwrap();
                from_js_value::<Baz>(location)
            })
            .map(|l| l.into())
            .collect())
    }
}

#[wasm_bindgen(js_name = "useExternVec")]
pub fn use_extern_vec(vec: ArrayBaz) {
    let vec: Vec<Baz> = vec.try_into().unwrap();
    console::log_1(&format!("Rust: use_extern_vec {:?}", vec).into());
}

// Other things I tried so far:
// - A custom `BazVec<'a>(Vec<&'a Baz>)` type. Doesnt work because wasm-bindgen
//   cannot bind types with lifetime parameters.
// - Wrapping the Baz instances in a handle that is `Copy`. They don't get copied
//   when going across the boundary, so it doesn't work.
