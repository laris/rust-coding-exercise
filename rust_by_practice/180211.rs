use std::collections::HashMap;

fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    //let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    let folks: HashMap<_, _> = IntoIterator::into_iter(names).zip(IntoIterator::into_iter(ages)).collect();
    println!("{folks:?}");
}
