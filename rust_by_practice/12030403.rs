fn main() {

    let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    let num = unsafe { std::mem::transmute::<[u8;4], u32>(raw_bytes)};

    let num = u32::from_ne_bytes(raw_bytes);
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    let ptr = &0;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr)};

    let ptr_num_cast = ptr as *const i32 as usize;

    let ptr = &mut 0;
    let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr)};
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32)};

    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust")};
    assert_eq!(slice, &[82, 117, 115, 116]);

    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    assert_eq!(b"Rust", &[82, 117, 115, 116]);

// --------------------------------------------------------------------------------
    /*Turning raw bytes(&[u8]) to u32, f64, etc.: */
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    /*Turning a pointer into a usize: */
    let ptr = &0;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };

    // Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;

    /*Turning an &mut T into an &mut U: */
    let ptr = &mut 0;
    let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };

    // Now, put together `as` and reborrowing - note the chaining of `as`
    // `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };

    /*Turning an &str into a &[u8]: */
    // this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);

    // You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or, just use a byte string, if you have control over the string
    // literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}
