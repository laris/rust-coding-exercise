fn main() {
    // checked_*
    let n: Option<u32> = 0u32.checked_sub(1);       // None
    let n: Option<u32> = u32::MAX.checked_add(1);   // None
    let n: Option<u32> = 9_u32.checked_add(1);      // some(10)
    // overflowing_*
    let n: (u32, bool) = 0u32.overflowing_sub(1);
    println!("0x{:x}", n.0);
    let n: (u32, bool) = 5u32.overflowing_add(1);
    // saturating_*
    let n: u32 = 0_u32.saturating_sub(9001); // 0
    let n: u32 = u32::MAX.saturating_add(u32::MAX); // 0xffff_ffff
    println!("0x{:x}", n);
    //wrapping_*
    let n: u32 = 1_u32.wrapping_sub(2);
    println!("0x{:x}", n);
    let n: u32 = u32::MAX.wrapping_add(1);
    println!("0x{:x}", n);
}
