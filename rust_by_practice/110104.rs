/*
use utf8_slice;
fn main() {
   let s = "The 🚀 goes to the 🌑!";

   let rocket = utf8_slice::slice(s, 4, 5);
   // Will equal "🚀"
}
*/

fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 { assert_eq!(c, '世') }
    }

    println!("OK");
}
