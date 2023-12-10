fn main() {
    let numbers: Vec<u32> = vec![1, 2, 3];
    let numbers: Vec<_> = vec![1, 2, 3];
    //let odds: Vec<_> = numbers
    let odds = numbers
        .iter()
        .filter(|n| **n % 2 == 1)
        //.collect();
        .collect::<Vec<_>>();
    println!("{odds:?}");
}
