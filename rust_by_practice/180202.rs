fn main() {
    let mut v = Vec::new();
        for n in 1..=100 {
            v.push(n);
        }
    println!("{v:?}");
    assert_eq!(v.len(), 100);

}
