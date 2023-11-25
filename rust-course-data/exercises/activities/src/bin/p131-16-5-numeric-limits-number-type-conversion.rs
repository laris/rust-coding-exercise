use std::convert::TryFrom;
#[allow(unused_variables)]
fn main() {
    let a = 15u8 as u16;
    let b = a as u8 + 20u16 as u8;

    let c = 600_u16 as u8; // =88
        //  600 - 256 = 344
        //  344 - 256 = 88
    let d = u8::try_from(300u16);
    println!("{d:?}");
}
