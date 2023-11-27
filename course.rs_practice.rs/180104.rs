fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n = example_closure("5".to_string());
}
