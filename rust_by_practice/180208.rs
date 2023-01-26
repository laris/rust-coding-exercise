fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.clone().sum();
    assert_eq!(total, 6);
    println!("{v1:?}, {v1_iter:?}");
}
