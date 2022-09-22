#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");

    // f64 and f32
    let x      = 2.0;    // f64
    let y: f32 = 3.0;    // f32

    // basic math operator
    let sum = 5 + 10;   // add
    let dif = 95.5 - 4.3; // minus
    let product = 4 * 30; // mul
    let quotient = 56.7 / 32.2; // div
    let remainder = 43 % 5; // mod div

    // 
    let mut sum = 15;
    sum += 1;

    // bool
    let bool_true = true;
    let bool_false = false;

    // char
    let ch = 'c';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 = 500
    // tup.1 = 6.4
    // tup.2 = 1
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // array
    let array = [1, 2, 3, 4, 5];    // len = 5 array with int
    let month = ["Jan", "Feb", "Mar"]; // len = 3 string array
    let array_i32 = [1, 2, 3, 4, 5]; // i32
    let array_same = [3; 5]; // [3, 3, 3, 3, 3]
    let first = array[0]; // access array element
    let second = array[1]; 
    println!("array's first element is: {}", first);

    // array[0] = 123;
    // error[E0594]: cannot assign to `array[_]`, 
    // as `array` is not declared as mutable

    let mut array = [1, 2, 3];
    array[0] = 4;
    println!("array's first elemnt is: {}", array[0]);

}
