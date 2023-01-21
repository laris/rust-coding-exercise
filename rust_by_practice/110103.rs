fn main() {
    let s: String = String::from("hello, world!"); // 1 x heap alloc

    let slice: &str = &s; // no heap alloc
    let s: String = slice.to_string(); // 2 x heap alloc
    assert_eq!(s, "hello, world!");
    println!("Ok");
}
