#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

mod greet {
    use std::collections::HashMap;
    pub fn fn_demo() {
        let mut hm_in_greet = HashMap::new();
        hm_in_greet.insert("name", 1);
    }
    pub fn hello() { println!("hello"); }
    pub fn goodbye() { println!("goodbye"); }
}


mod math {
    /* below will report HashMap cannot found
     * because no import HashMap
    pub fn fn_demo() {
        let mut hm_in_math = HashMap::new();
        hm_in_math.insert("name", 1);
    }
    */
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn sub(a: i32, b: i32) -> i32 { a - b }
}

fn main() {
    use greet::hello;
    hello();
    greet::goodbye();

    use math::add;
    add(2, 1);
    math::sub(2, 1);

    let mut hm_in_main = HashMap::new();
    hm_in_main.insert("name", 1);
}
