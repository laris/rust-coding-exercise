fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{count}");
        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }
    assert_eq!(count, 5);
}
