use std::mem;

fn main() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));

    let b;
    //let b = b = b = true;
    //let b = (b = (b = true));

}
