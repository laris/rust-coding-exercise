#![allow(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
    assert_eq!(v, 0b1110_1000);
}
