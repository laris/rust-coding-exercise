fn main() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{}, {}", v1, v2);
    let v3 = u8::wrapping_add(255, 1);
    let v4 = u8::checked_add(255, 1);
    let v5 = u8::overflowing_add(255, 1);
    let v6 = u8::saturating_add(255, 1);
    println!("{v3:?}, {v4:?}, {v5:?}, {v6:?}");
}
