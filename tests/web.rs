//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rust_wasm_benchmark_code::sum_double;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let array = [1.0, 2.0, 3.0];
    assert_eq!(sum_double(&array, 3), 6.0);
}
