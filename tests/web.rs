//! Test suite for the Web and headless browsers.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use collisionDetectionRust::Position;
use collisionDetectionRust::collision_detection;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let p1 = Position::new(1.0, 1.0, 1.0);
    let p2 = Position::new(2.0, 2.0, 2.0);
    let v = vec![p1, p2];
    let test = JsValue::from_serde(&v).unwrap();
    let radiuses: [f64;2] = [5.0, 5.0];
    let mut res: [i64;2] = [0, 0];
    let result = collision_detection(&test, &radiuses, &mut res,2);
    assert_eq!(result, 1);
}
