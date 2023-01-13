fn main() {
    //let s = "hello, world".to_string();
    //let s = "hello, world".into();
    let s = String::from("hello, world");
    greetings(s);
}

fn greetings(s: String) {
    println!("{s}");
}
