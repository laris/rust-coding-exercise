fn main() {
    let s = String::from("hello world");
    print_str1(s.clone());
    println!("{s}");

    print_str2(&s);
    println!("{s}");
}

fn print_str1(s: String) {
    println!("{s}");
}
fn print_str2(s: &String) {
    println!("{s}");
}
