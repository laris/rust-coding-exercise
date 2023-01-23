use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    //n_str.parse::<i32>().map(|x| x + 2)
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
