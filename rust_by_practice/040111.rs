fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8  - 2 == -1);
    assert!(1u8  - 1 == 0);
    assert!(3 * 50 == 150);
    assert!(9.6f32 / 3.2 == 3.0);
    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011  OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {:08b}", 1u8 << 5);
    print!("0b1000 0x8 >> 3 is {:04b} 0x{0:02x}", 0x8u8 >> 3);
}
