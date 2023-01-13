fn main() {
    let x = String::from("hello world");
    //let y = x.clone();
    //let y = &x;
    let y = &x[..];
    println!("{x}, {y}");

    let x = &String::from("hello world");
    let y = x;
    println!("{x}, {y}");

    let x = "hello world";
    let y = x;
    println!("{x}, {y}");

    let x = 10;
    let y = x;
    println!("{x}, {y}");
}
