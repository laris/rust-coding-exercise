use std::str::FromStr;

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();

    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);
}
