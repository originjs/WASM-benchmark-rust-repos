use sha3::*;

fn main() {
    let dataWords = [16777216_u32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128];
    let mut stateData = [0_u32;50];
    doCrypt(0, &dataWords, 72, 18, &mut stateData, 0);
}