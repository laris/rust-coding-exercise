
fn iter_bytes<T: AsRef<[u8]>>(arg: T) {
    for i in arg.as_ref() {
        println!("{}", i);
    }
    println!("");
}

fn main() {
    println!("Hello, world!");
/*
    let s: String = String::from("this is a string");
    let v: Vec<u8> = vec![1, 2, 3];
    let c: &str = "hello";
*/
    let s: String = String::from("123");
    let v: Vec<u8> = vec![1, 2, 3];
    let c: &str = "123";
    // like c++ overload (?), impl via fan-xing, arg need restricted by trait
    iter_bytes(s);
    iter_bytes(v);
    iter_bytes(c);
}
