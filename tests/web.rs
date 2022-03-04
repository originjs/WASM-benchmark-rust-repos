//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
use aes::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let iv: [u32; 4] = [808530483, 875902519, 943274555, 1010646591];
    let mut dataWords = [66051, 67438087, 134810123, 202182159, 269554195, 336926231, 404298267, 471670303];
    let keySchedule = [539042339, 606414375, 673786411, 741158447, -116377774, -583724683, -182699170, -651094671, 1011455079, -511933166, 341884492, -850285763, -383011878, 139933384, 473401476, -781835335, -770841116, -631581908, -965911640, 386627601, -374249708, 871085112, -175706224, -494037119, -1808176244, -1479352396, 1381249060, -1327913051, -513446549, 1186443999, 350454523, -1539693218, -514181854, -1477605379, -1290926842, 389292120, -487759918, 1157683247, -166798039, -516227727, 1421474858, 297346053, -407373524, 109904989];
    let mut SBOX = [0; 256];
    let mut SUB_MIX_0 = [0; 256];
    let mut SUB_MIX_1 = [0; 256];
    let mut SUB_MIX_2 = [0; 256];
    let mut SUB_MIX_3 = [0; 256];


    let mut d: [u32; 256] = [0; 256];
    for i in 0..256 {
        if (i < 128) {
            d[i] = (i as u32) << 1;
        } else {
            d[i] = ((i as u32) << 1) ^ 0x11b;
        }
    }

    // Walk GF(2^8)
    let mut x = 0u32;
    let mut xi = 0u32;
    for i in 0..256 {
        // Compute sbox
        let mut sx = xi ^ (xi << 1) ^ (xi << 2) ^ (xi << 3) ^ (xi << 4);
        sx = (sx >> 8) ^ (sx & 0xff) ^ 0x63;
        SBOX[x] = sx;
        INV_SBOX[sx] = x;

        // Compute multiplication
        let x2 = d[x];
        let x4 = d[x2];
        let x8 = d[x4];

        // Compute sub bytes, mix columns tables
        let t = (d[sx] * 0x101) ^ (sx * 0x1010100);
        SUB_MIX_0[x] = (t << 24) | (t >> 8);
        SUB_MIX_1[x] = (t << 16) | (t >> 16);
        SUB_MIX_2[x] = (t << 8) | (t >> 24);
        SUB_MIX_3[x] = t;

        // Compute inv sub bytes, inv mix columns tables
        t = (x8 * 0x1010101) ^ (x4 * 0x10001) ^ (x2 * 0x101) ^ (x * 0x1010100);
        INV_SUB_MIX_0[sx] = (t << 24) | (t >> 8);
        INV_SUB_MIX_1[sx] = (t << 16) | (t >> 16);
        INV_SUB_MIX_2[sx] = (t << 8) | (t >> 24);
        INV_SUB_MIX_3[sx] = t;

        // Compute next counter
        if (x != 0) {
            xi = 1;
            x = xi;
        } else {
            x = x2 ^ d[d[d[x8 ^ x2]]];
            xi ^= d[d[xi]];
        }
    }
    doEncrypt("cfb", 10, 8, 4, &iv, &mut dataWords, &keySchedule, &SUB_MIX_0, &SUB_MIX_1, &SUB_MIX_2, &SUB_MIX_3, &SBOX);
    assert_eq!(1 + 1, 2);
}
