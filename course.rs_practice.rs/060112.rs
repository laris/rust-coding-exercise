fn main() {
    for c in "你好，世界".chars() {
        print!("{c}");
    }
    println!("");
}
/*
use utf8_slice;
fn main() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // 结果是 "🚀"
*/
