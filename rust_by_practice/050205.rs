fn main() {
    let mut s = String::from("hello, ");
    let p = &mut s;
    p.push_str("world");
}
