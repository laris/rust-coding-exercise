fn main() {
    let mut s = String::new();
    let update_string = move |str| -> String { s.push_str(str); s };
    exec(update_string);
    println!("{s}");
}

fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) { f("hello") }
