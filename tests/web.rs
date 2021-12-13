//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rust_wasm_benchmark_code::multiply_double_vec;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let data_size = 3;
    let src1 = [1.0, 3.0, 5.0];
    let src2 = [2.0, 4.0, 6.0];
    let mut res = [0.0, 0.0, 0.0];
    multiply_double_vec(&src1, &src2, &mut res, data_size);
    assert_eq!(res, [2.0, 12.0, 30.0])
}
