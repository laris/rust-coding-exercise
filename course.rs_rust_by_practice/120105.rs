fn main() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);

    // convert array's reference to raw ref (include ptr + len)
    // T=u64, known size slice, raw ptr, like &arr[..]
    let a: *const [u64] = &arr;
    println!("{}", std::mem::size_of_val(&a));
    let b = a as *const [u8]; // convert T=u8
    println!("{}", std::mem::size_of_val(&b));
    unsafe { assert_eq!(std::mem::size_of_val(&*b), 1*13);}
    println!("Ok");
}
