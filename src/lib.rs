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
pub fn multiply_int_vec(src1: &[i32], src2: &[i32], res: &mut [i32], n: usize) {
    for i in 0..n {
        res[i] = src1[i] * src2[i];
    }
}
