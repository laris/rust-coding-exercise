fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
    let s: Box<&str> = "hello, world".into();
    //let s: Box<&str> = Box::new("hello, world");
    greetings(*s);
    let s = String::from("hello, world");
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{s}");
}
