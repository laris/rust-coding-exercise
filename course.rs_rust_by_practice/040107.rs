#![feature(type_name_of_val)]
#[allow(unused_variables)]
fn main() {
    let x = 1_000.000_1; // f64
    println!("{}", std::any::type_name_of_val(&x));
    let y: f32 = 0.12;
    let z = 0.01_f64;
}
