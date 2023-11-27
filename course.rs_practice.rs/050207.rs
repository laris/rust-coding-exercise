fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}");
}
