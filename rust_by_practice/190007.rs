fn my_function(n: usize) -> [u32; usize] {
    [123; n]
}

fn main() {
    let arr = my_function();
    println!("{arr:?}");
}
