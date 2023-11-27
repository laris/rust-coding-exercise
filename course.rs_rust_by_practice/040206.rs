fn main() {
    let unit: () = ();
    assert!(std::mem::size_of_val(&unit) == 0);
    println!("Success!");
}
