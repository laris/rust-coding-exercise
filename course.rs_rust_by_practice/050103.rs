fn main() {
    let s = give_ownership();
    println!("{s}");
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    //let _s = s.into_bytes();
    let _s = s.as_bytes();
    s
}
