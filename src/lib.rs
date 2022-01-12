mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate serde_json;
extern crate wasm_bindgen;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn imageConvolute(data: &[u8], data2: &mut [u8], width: i32, height: i32, weights: &[f64], wwidth: i32, wheight: i32) {
    let halfWWidth = wwidth / 2;
    let halfWHeight = wheight / 2;

    for y in 0..height {
        for x in 0..width {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;
            let mut a: f64 = 0.0;

            for wy in 0..wheight {
                let sy = y + wy - halfWHeight;
                if sy < 0 || sy >= height {
                    continue;
                }

                for wx in 0..wwidth {
                    let sx = x + wx - halfWWidth;
                    if sx < 0 || sx >= width {
                        continue;
                    }

                    let index = (sy * width + sx) as usize;
                    let weight: f64 = weights[(wy * wwidth + wx) as usize];
                    r += f64::from(data[index * 4 + 0]) * weight;
                    g += f64::from(data[index * 4 + 1]) * weight;
                    b += f64::from(data[index * 4 + 2]) * weight;
                    a += f64::from(data[index * 4 + 3]) * weight;
                }
            }

            let index = (y * width + x) as usize;
            data2[index * 4 + 0] = r as u8;
            data2[index * 4 + 1] = g as u8;
            data2[index * 4 + 2] = b as u8;
            data2[index * 4 + 3] = a as u8;
        }
    }
}
