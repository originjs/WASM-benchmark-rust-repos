//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rust_wasm_benchmark_code::multiply_int_vec;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let data_size = 3;
    let src1 = [1, 3, 5];
    let src2 = [2, 4, 6];
    let mut res = [0, 0, 0];
    multiply_int_vec(&src1, &src2, &mut res, data_size);
    assert_eq!(res, [2, 12, 30])
}
