use aes::*;

fn initData() {}

fn main() {
    let iv: [u32; 4] = [808530483, 875902519, 943274555, 1010646591];
    let mut dataWords = [3682395770, 4047365498, 2514035429, 844807955, 1997576358, 3079451740, 43839936, 3723366171];

    let mut SBOX = [0; 256];
    let mut INV_SBOX = [0; 256];
    let mut SUB_MIX_0 = [0; 256];
    let mut SUB_MIX_1 = [0; 256];
    let mut SUB_MIX_2 = [0; 256];
    let mut SUB_MIX_3 = [0; 256];
    let mut INV_SUB_MIX_0 = [0; 256];
    let mut INV_SUB_MIX_1 = [0; 256];
    let mut INV_SUB_MIX_2 = [0; 256];
    let mut INV_SUB_MIX_3 = [0; 256];


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
        SBOX[x as usize] = sx;
        INV_SBOX[sx as usize] = x;

        // Compute multiplication
        let x2 = d[x as usize];
        let x4 = d[x2 as usize];
        let x8 = d[x4 as usize];

        // Compute sub bytes, mix columns tables
        let mut t = (d[sx as usize] * 0x101) ^ (sx * 0x1010100);
        SUB_MIX_0[x as usize] = (t << 24) | (t >> 8);
        SUB_MIX_1[x as usize] = (t << 16) | (t >> 16);
        SUB_MIX_2[x as usize] = (t << 8) | (t >> 24);
        SUB_MIX_3[x as usize] = t;

        // Compute inv sub bytes, inv mix columns tables
        t = (x8 * 0x1010101) ^ (x4 * 0x10001) ^ (x2 * 0x101) ^ (x * 0x1010100);
        INV_SUB_MIX_0[sx as usize] = (t << 24) | (t >> 8);
        INV_SUB_MIX_1[sx as usize] = (t << 16) | (t >> 16);
        INV_SUB_MIX_2[sx as usize] = (t << 8) | (t >> 24);
        INV_SUB_MIX_3[sx as usize] = t;

        // Compute next counter
        if (x == 0) {
            xi = 1;
            x = xi;
        } else {
            x = x2 ^ d[d[d[x8 as usize ^ x2 as usize] as usize] as usize];
            xi ^= d[d[xi as usize] as usize];
        }
    }
    // println!("SUB_MIX_0 : {:X?}", SUB_MIX_0);
    // println!("SUB_MIX_0 : {:#02X?}", SUB_MIX_0);
    // println!("SUB_MIX_1 : {:#02X?}", SUB_MIX_1);
    // println!("SUB_MIX_2 : {:#02X?}", SUB_MIX_2);
    // println!("SUB_MIX_3 : {:#02X?}", SUB_MIX_3);
    // println!("INV_SUB_MIX_0 : {:#02X?}", INV_SUB_MIX_0);
    // println!("INV_SUB_MIX_1 : {:#02X?}", INV_SUB_MIX_1);
    // println!("INV_SUB_MIX_2 : {:#02X?}", INV_SUB_MIX_2);
    // println!("INV_SUB_MIX_3 : {:#02X?}", INV_SUB_MIX_3);


    let keySize = 4u32;
    let mut t;
    let keyWords = [539042339, 606414375, 673786411, 741158447];
    let mut keySchedule: [u32; 44] = [0u32; 44];
    let RCON = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
    let ksRows = 44;
    for ksRow in 0..ksRows {
        if (ksRow < keySize) {
            keySchedule[ksRow as usize] = keyWords[ksRow as usize];
        } else {
            t = keySchedule[ksRow as usize - 1];

            if ((ksRow % keySize) == 0) {
                // Rot word
                t = (t << 8) | (t >> 24);

                // Sub word
                t = (SBOX[t as usize >> 24] << 24)
                    | (SBOX[(t >> 16) as usize & 0xff] << 16)
                    | (SBOX[(t >> 8) as usize & 0xff] << 8)
                    | SBOX[t as usize & 0xff];

                // Mix Rcon
                t ^= RCON[(ksRow / keySize) as usize] << 24;
            } else if (keySize > 6 && ksRow % keySize == 4) {
                // Sub word
                t = (SBOX[t as usize >> 24] << 24)
                    | (SBOX[(t >> 16) as usize & 0xff] << 16)
                    | (SBOX[(t >> 8) as usize & 0xff] << 8)
                    | SBOX[t as usize & 0xff];
            }

            keySchedule[ksRow as usize] = keySchedule[ksRow as usize - keySize as usize] ^ t;
        }
    }

    // Compute inv key schedule
    let mut invKeySchedule = [0u32; 44];
    for invKsRow in 0..ksRows {
        let ksRow = ksRows - invKsRow;

        if (invKsRow % 4 != 0) {
            t = keySchedule[ksRow as usize];
        } else {
            t = keySchedule[ksRow as usize - 4];
        }

        if (invKsRow < 4 || ksRow <= 4) {
            invKeySchedule[invKsRow as usize] = t;
        } else {
            invKeySchedule[invKsRow as usize] = INV_SUB_MIX_0[SBOX[t as usize >> 24] as usize]
                ^ INV_SUB_MIX_1[SBOX[(t >> 16) as usize & 0xff] as usize]
                ^ INV_SUB_MIX_2[SBOX[(t >> 8) as usize & 0xff] as usize]
                ^ INV_SUB_MIX_3[SBOX[t as usize & 0xff] as usize];
        }
    }

    // doEncrypt("cfb", 10, 8, 4, &iv, &mut dataWords, &keySchedule);
    // println!("encrypt : {:?}", dataWords);
    // doDecrypt("cfb", 10, 8, 4, &iv, &mut dataWords, &invKeySchedule, &INV_SUB_MIX_0, &INV_SUB_MIX_1, &INV_SUB_MIX_2, &INV_SUB_MIX_3, &INV_SBOX);
    let keyWords = [539042339, 606414375, 673786411, 741158447];
    doEncrypt("ecb", 10, 8, 4, &iv, &mut dataWords, 4, &keyWords);
    // doDecrypt("ecb", 10, 8, 4, &iv, &mut dataWords, &keySchedule);
    println!("decrypt : {:?}", dataWords);
    println!("{:?}", dataWords);
}