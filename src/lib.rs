use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn doEncrypt(
    mode: &str,
    nWordsReady: usize,
    blockSize: usize,
    iv: &[u32],
    dataWords: &mut [u32],
    subKeys: &[u32],
    SBOX_P_ARRAY: &[u32],
    SBOX_MASK: &[u32],
) {
    let mut SBOX_P = Vec::new();
    for i in 0..8 {
        let mut map = HashMap::new();
        for j in 0..64 {
            map.insert(
                SBOX_P_ARRAY[i * 128 + j * 2],
                SBOX_P_ARRAY[i * 128 + j * 2 + 1],
            );
        }
        SBOX_P.push(map);
    }
    if nWordsReady > 0 {
        let mut offset: usize = 0;
        let mut prevBlock = slice(iv, 0, blockSize);
        if mode == "cbc" {
            while offset < nWordsReady {
                xorBlock(blockSize, prevBlock, dataWords, offset);
                doCryptBlock(dataWords, offset, subKeys, &SBOX_P, SBOX_MASK);
                prevBlock = slice(dataWords, offset, offset + blockSize);
                offset += blockSize;
            }
        }
    }
}

#[wasm_bindgen]
pub fn doDecrypt(
    mode: &str,
    nWordsReady: usize,
    blockSize: usize,
    iv: &[u32],
    dataWords: &mut [u32],
    subKeys: &[u32],
    SBOX_P_ARRAY: &[u32],
    SBOX_MASK: &[u32],
) {
    let mut SBOX_P = Vec::new();
    for i in 0..8 {
        let mut map = HashMap::new();
        for j in 0..64 {
            map.insert(
                SBOX_P_ARRAY[i * 128 + j * 2],
                SBOX_P_ARRAY[i * 128 + j * 2 + 1],
            );
        }
        SBOX_P.push(map);
    }
    if nWordsReady > 0 {
        let mut offset: usize = 0;
        let mut prevBlock = slice(iv, 0, blockSize);
        if mode == "cbc" {
            while offset < nWordsReady {
                let thisBlock = slice(dataWords, offset, offset + blockSize);
                doCryptBlock(dataWords, offset, subKeys, &SBOX_P, SBOX_MASK);
                xorBlock(blockSize, prevBlock, dataWords, offset);
                prevBlock = thisBlock;
                offset += blockSize;
            }
        }
    }
}

#[wasm_bindgen]
pub fn tripleEncrypt(
    mode: &str,
    nWordsReady: usize,
    blockSize: usize,
    iv: &[u32],
    dataWords: &mut [u32],
    subKeys1: &[u32],
    subKeys2: &[u32],
    subKeys3: &[u32],
    SBOX_P_ARRAY: &[u32],
    SBOX_MASK: &[u32],
) {
    let mut SBOX_P = Vec::new();
    for i in 0..8 {
        let mut map = HashMap::new();
        for j in 0..64 {
            map.insert(
                SBOX_P_ARRAY[i * 128 + j * 2],
                SBOX_P_ARRAY[i * 128 + j * 2 + 1],
            );
        }
        SBOX_P.push(map);
    }
    if nWordsReady > 0 {
        let mut offset: usize = 0;
        let mut prevBlock = slice(iv, 0, blockSize);
        if mode == "cbc" {
            while offset < nWordsReady {
                xorBlock(blockSize, prevBlock, dataWords, offset);
                doCryptBlock(dataWords, offset, subKeys1, &SBOX_P, SBOX_MASK);
                doCryptBlock(dataWords, offset, subKeys2, &SBOX_P, SBOX_MASK);
                doCryptBlock(dataWords, offset, subKeys3, &SBOX_P, SBOX_MASK);
                prevBlock = slice(dataWords, offset, offset + blockSize);
                offset += blockSize;
            }
        }
    }
}

#[wasm_bindgen]
pub fn tripleDecrypt(
    mode: &str,
    nWordsReady: usize,
    blockSize: usize,
    iv: &[u32],
    dataWords: &mut [u32],
    subKeys1: &[u32],
    subKeys2: &[u32],
    subKeys3: &[u32],
    SBOX_P_ARRAY: &[u32],
    SBOX_MASK: &[u32],
) {
    let mut SBOX_P = Vec::new();
    for i in 0..8 {
        let mut map = HashMap::new();
        for j in 0..64 {
            map.insert(
                SBOX_P_ARRAY[i * 128 + j * 2],
                SBOX_P_ARRAY[i * 128 + j * 2 + 1],
            );
        }
        SBOX_P.push(map);
    }
    if nWordsReady > 0 {
        let mut offset: usize = 0;
        let mut prevBlock = slice(iv, 0, blockSize);
        if mode == "cbc" {
            while offset < nWordsReady {
                let thisBlock = slice(dataWords, offset, offset + blockSize);
                doCryptBlock(dataWords, offset, subKeys1, &SBOX_P, SBOX_MASK);
                doCryptBlock(dataWords, offset, subKeys2, &SBOX_P, SBOX_MASK);
                doCryptBlock(dataWords, offset, subKeys3, &SBOX_P, SBOX_MASK);
                xorBlock(blockSize, prevBlock, dataWords, offset);
                prevBlock = thisBlock;
                offset += blockSize;
            }
        }
    }
}

fn slice(arr: &[u32], start: usize, end: usize) -> Vec<u32> {
    let mut vec = Vec::new();
    for i in start..end {
        vec.push(arr[i]);
    }
    vec
}

fn xorBlock(blockSize: usize, block: Vec<u32>, words: &mut [u32], offset: usize) {
    // XOR blocks
    for i in 0..blockSize {
        words[offset + i] ^= &block[i];
    }
}

fn doCryptBlock(
    dataWords: &mut [u32],
    offset: usize,
    subKeys: &[u32],
    SBOX_P: &Vec<HashMap<u32, u32>>,
    SBOX_dataWordsASK: &[u32],
) {
    // Get input
    let mut lBlock = dataWords[offset];
    let mut rBlock = dataWords[offset + 1];

    // Initial permutation
    exchangeLR(4, 0x0f0f0f0f, &mut lBlock, &mut rBlock);
    exchangeLR(16, 0x0000ffff, &mut lBlock, &mut rBlock);
    exchangeRL(2, 0x33333333, &mut lBlock, &mut rBlock);
    exchangeRL(8, 0x00ff00ff, &mut lBlock, &mut rBlock);
    exchangeLR(1, 0x55555555, &mut lBlock, &mut rBlock);

    // Rounds
    for round in 0..16 {
        // Feistel function
        let mut f = 0;
        for i in 0..8 {
            f |= *&SBOX_P[i]
                .get(&((rBlock ^ subKeys[round * 8 + i]) & SBOX_dataWordsASK[i]))
                .unwrap();
        }
        let t = lBlock;
        lBlock = rBlock;
        rBlock = t ^ f;
    }

    // Undo swap from last round
    let t = lBlock;
    lBlock = rBlock;
    rBlock = t;

    // Final permutation
    exchangeLR(1, 0x55555555, &mut lBlock, &mut rBlock);
    exchangeRL(8, 0x00ff00ff, &mut lBlock, &mut rBlock);
    exchangeRL(2, 0x33333333, &mut lBlock, &mut rBlock);
    exchangeLR(16, 0x0000ffff, &mut lBlock, &mut rBlock);
    exchangeLR(4, 0x0f0f0f0f, &mut lBlock, &mut rBlock);

    // Set output
    dataWords[offset] = lBlock;
    dataWords[offset + 1] = rBlock;
}

// Swap bits across the left and right words
fn exchangeLR(offset: u32, mask: u32, lBlock: &mut u32, rBlock: &mut u32) {
    let t = ((*lBlock >> offset) ^ *rBlock) & mask;
    *rBlock ^= t;
    *lBlock ^= t << offset;
}

fn exchangeRL(offset: u32, mask: u32, lBlock: &mut u32, rBlock: &mut u32) {
    let t = ((*rBlock >> offset) ^ *lBlock) & mask;
    *lBlock ^= t;
    *rBlock ^= t << offset;
}
