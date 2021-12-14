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
pub fn sum_double(array: &[f64], n: usize) -> f64 {
    let mut s = 0.0;
    for i in 0..n {
        s += array[i];
    }
    return s;
}
