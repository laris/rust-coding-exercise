use std::num::ParseIntError as Error;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, Error> {
    Ok(n1_str.parse::<i32>()? * n2_str.parse::<i32>()?)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}
