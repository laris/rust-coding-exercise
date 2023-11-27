#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_parens)]

fn main() {
    println!("Hello, world!");

    another_function1();

    another_function2(5, 6);

    fn_expr_state();

    // complex expr
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x value is: {}", x);
    println!("y value is: {}", y);

    // nesting
    fn five() -> i32 {
        5
    }

    println!("five() value is: {}", five());

    // function return
    fn add1(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("{} add {} return {}", 5, 4, add1(5, 4));

    fn add2(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("{} add {} return {}", 5, 5, add2(5, 5));

    // fn fn_ret() -> i32 { return 1.0; }
    // fn fn_ret() -> i32 { 1.0 }
    // expected `i32`, found floating-point number
}

fn another_function1() {
    println!("Hello, runoob!");
}

fn another_function2(x: i32, y: i32) {
    println!("x value is: {}", x);
    println!("y value is: {}", y);
}

fn fn_expr_state() {
    let a = 6;
    // let a = (let b = 2);
    //          ^^^^^^^^^^
    // error: expected expression, found statement (`let`)

    let aa;
    let ret_val_1 = (aa = 7);
    println!("\"aa = 7\" return value is: {:?}", ret_val_1);

    let b = 1;
    let ret_val_2 = (b + 2);
    println!("\"b + 2\" return value is: {:?}", ret_val_2);

    let c = 2;
    let ret_val_3 = c * (a + b);
    println!("\"c * (a + b)\" return value is: {:?}", ret_val_3);
}