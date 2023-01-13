fn main() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
}

fn push_str(s: &mut String) {
    s.push_str("world");
}
