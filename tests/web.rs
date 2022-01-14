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
}
