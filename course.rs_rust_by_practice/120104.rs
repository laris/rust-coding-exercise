fn main() {
    let mut values: [i32; 2] = [1, 2]; // values is array var
    //let p0 = &mut values[..]; // p0 is slice
    let p0 = &mut values;
    let p1: *mut i32 = p0.as_mut_ptr(); // as_mut_ptr(&mut self) -> *mut T
    //let p1: *mut i32 = values.as_mut_ptr();

    let first_addr: usize = p1 as *const i32 as usize;
    let second_addr = first_addr + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_addr as *mut i32;
    unsafe { *p2 += 1 }

    assert_eq!(values[1], 3);
}
