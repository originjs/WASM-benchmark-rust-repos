mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn multiply_double(a: f64, b: f64, n: u32) -> f64 {
    let mut c = 1.0;
    for _ in 0..n {
        c = c * a * b;
    }
    return c;
}
