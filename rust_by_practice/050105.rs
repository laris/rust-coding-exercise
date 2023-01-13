fn main() {
    //let x = (1, 2, (), "hello".to_string());
    //let y = x.clone();
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{x:?}, {y:?}");
}
