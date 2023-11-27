fn main() {
    let x = 5;
    let p = &x;
    println!("x's memory addr: {p:p} {:p}", &x);
}
