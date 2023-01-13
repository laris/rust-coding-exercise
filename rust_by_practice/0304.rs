fn main() {
    println!("{}, world", define_x());
}

//fn define_x() -> &'static str {
fn define_x() -> String {
    let x = "hello";
    //x
    x.to_string()
}
