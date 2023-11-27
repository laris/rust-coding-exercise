fn main() {
    let mut s: String = "hello, ".to_string();
    //s.push_str("world".to_string());
    s.push_str("world");
    s.push('!');
    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");
    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s);
}
