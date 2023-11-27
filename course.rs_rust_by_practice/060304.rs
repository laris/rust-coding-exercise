fn main() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..=1];
    assert_eq!(slice1, slice2);
}
