fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += &"!".to_string();

    println!("{s}");
}
