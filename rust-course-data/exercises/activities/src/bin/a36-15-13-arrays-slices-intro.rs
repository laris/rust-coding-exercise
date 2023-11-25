#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    // create array
    let numbers = [1, 2, 3];
    let mut numbers: [u8; 3] = [1, 2, 3];
    let num_slice: &[u8] = numbers.as_slice();

    print_u8_arry(numbers); // array with u8 can be copied, so pass in the copied array
    print_u8_arry_slice(&numbers);
    print_u8_arry_slice(num_slice);
    print_u8_array_mut_slice(& mut numbers);

    let chars = vec!['A', 'B', 'C', 'D'];
    let bc = &chars[1..=2];
    let ab = &chars[0..2];
    print_char_slice(ab);
    print_char_slice(bc);
}

// function pass arg
// move
fn print_u8_arry(arr: [u8; 3]) {
    println!("{arr:?}");
}
// borrow
fn print_u8_arry_slice(arr: &[u8]) {
    println!("{arr:?}");
}
// mut borrow
fn print_u8_array_mut_slice(arr: &mut [u8]) {
    println!("{arr:?}");
}

fn print_char_slice(arr: &[char]) {
    println!("{arr:?}");
}
