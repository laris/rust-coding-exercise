fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        //&mut value => value.push_str(" world!")
        value => { 
            value.push_str(" world!");
            println!("{value}");
            }
        _ => {}
    }

    println!("{v}");
}
