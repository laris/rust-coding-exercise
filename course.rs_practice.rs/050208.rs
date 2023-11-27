fn main() {
    let mut s = String::from("hello, ");
    borrow_object(&mut s);
}
fn borrow_object(s: &mut String) {}
