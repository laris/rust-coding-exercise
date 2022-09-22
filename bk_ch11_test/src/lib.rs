pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two2(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
pub fn greeting2(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

pub struct Guess2 {
    value: i32,
}
impl Guess2 {
    pub fn new(value: i32) -> Guess2 {
        if value < 1 { panic!("Guess value must be greater than or equal to 1, got {}.", value); }
        else
        if value > 100 { panic!("Guess value must be less than or equal to 100, got {}.", value); }
        Guess2 { value }
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    print!("I got the value {}.", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_panic() {
        //panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_fail() {
        assert_ne!(4, add_two2(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    #[test]
    #[should_panic]
    fn greeting_contains_name2() {
        let result = greeting2("Carol");
        assert!(result.contains("Carol"));
    }
    #[test]
    #[should_panic]
    fn greeting_contains_name3() {
        let result = greeting2("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    //#[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[should_panic(expected = "Guess value must be less than or equal to")]
    fn greater_than_100_2() {
        Guess2::new(200);
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn it_works_match_result() -> Result<(), String> {
        let v: Result<i32, String> = Ok(4);
        match v {
            Ok(_) => {Ok(())},
            Err(_) => {Err(String::from("err"))},
        }
    }
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_pass2() {
        let value = prints_and_returns_10(8);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        //code that takes an hour to run
    }

}