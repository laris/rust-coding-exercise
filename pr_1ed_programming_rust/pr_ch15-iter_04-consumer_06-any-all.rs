fn main() {
    let id = "Iterator";
    assert!( id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));
}
