fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    //r1.push_str("world");
    r2.push_str("world");
}
