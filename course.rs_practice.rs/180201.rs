fn main() {
    let arr = [0; 10];
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    for a in arr.iter() {
        println!("{a}");
    }
}
