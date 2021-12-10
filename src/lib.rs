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
pub fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
