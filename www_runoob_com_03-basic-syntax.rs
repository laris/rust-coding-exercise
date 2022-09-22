#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    // variable
    let a = 123;
    // a = "abc";  // expected integer, found `&str`
    // a = 4.56;   // expected integer, found floating-point number
    // a = 456;

    // mutable variable
    let mut a = 123;
    a = 456;

    let a = 123;
    let a = 456;

    const b: i32 = 123;
    // let b = 456;    // interpreted as a constant pattern, not a new variable

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut s = "123";
    // s = s.len();    // expected `&str`, found `usize`
}
