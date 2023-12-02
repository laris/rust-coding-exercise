fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Jayson";
    let add = |a, b| a + b;
    let add: Box<_> = Box::new(add);
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    let div = Box::new(move |a, b| {
        println!("divide! {name}!");
        a / b
    });
    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 3, mul));
    println!("{}", math(2, 2, div));
}
