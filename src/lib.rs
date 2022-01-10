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

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Position {
        Position { x: x, y: y, z: z }
    }
}

#[wasm_bindgen]
pub fn collision_detection(js_positions: &JsValue, radiuses: &[f64], res: &mut [u8], n: usize) -> i32 {
    let positions: Vec<Position> = js_positions.into_serde().unwrap();

    let mut count = 0;
    for (i, p) in positions.iter().enumerate() {
        let r = radiuses[i];
        let mut collision = false;

        for j in (i + 1)..n {
            let p2 = &positions[j];
            let r2 = radiuses[j];
            let dx = p.x - p2.x;
            let dy = p.y - p2.y;
            let dz = p.z - p2.z;
            let mut d = (dx * dx + dy * dy + dz * dz).sqrt();

            if (r > d) {
                collision = true;
                count += 1;
                break;
            }
        }

        let index = (i / 8);
        let pos = 7 - (i % 8);
        if (!collision) {
            res[index] &= !(1 << pos);
        } else {
            res[index] |= (1 << pos);
        }
    }

    count
}
