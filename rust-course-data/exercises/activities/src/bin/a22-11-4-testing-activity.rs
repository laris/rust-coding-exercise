// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp_lower() {
        let input = (10, 100, 1000);
        let result = clamp(input.0, input.1, input.2);
        let expected = 100;
        assert_eq!(result, expected, "n={}, lower={}, upper={}, error parse", input.0, input.1, input.2);
    }
    #[test]
    fn check_clamp_upper() {
        let input = (5000, 100, 1000);
        let result = clamp(input.0, input.1, input.2);
        let expected = 1000;
        assert_eq!(result, expected, "n={}, lower={}, upper={}, error parse", input.0, input.1, input.2);
    }
    #[test]
    fn check_clamp_mid() {
        let input = (1, 0, 255);
        let result = clamp(input.0, input.1, input.2);
        let expected = 1;
        assert_eq!(result, expected, "n={}, lower={}, upper={}, error parse", input.0, input.1, input.2);
    }
    #[test]
    fn check_div_50_1() {
        let result = div(50, 10);
        let expected = Some(5);
        assert_eq!(result, expected);
    }
    #[test]
    fn check_div_50_zero() {
        let result = div(50, 0);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_concat_str() {
        let result = concat("hello", "world!");
        let expected = "hello world!".to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn check_concat_number_str() {
        let result = concat("123", "4567".to_owned().as_str());
        let expected = "123 4567".to_string();
        assert_eq!(result, expected);
    }
}
