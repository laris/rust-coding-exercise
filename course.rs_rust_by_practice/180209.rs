use std::collections::HashMap;
fn main() {
    let names = [("Sunface", 18), ("sunfei", 18)];
    //let folks: HashMap<_, _> = names.into_iter().collect(); // 2021 works
    let folks: HashMap<_, _> = IntoIterator::into_iter(names).collect(); // 2015 works

    println!("{folks:?}");

    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<i32> = v1.into_iter().collect();
    assert_eq!(v2, vec![1, 2, 3]);

    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<&i32> = v1.iter().collect();
    assert_eq!(v2, vec![&1, &2, &3]);
}
