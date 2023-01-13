fn main() {
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
}

fn borrow_object(s: &String) {}
