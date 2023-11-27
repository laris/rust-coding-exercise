#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(t: i32) -> Self {
        Self { value: t }
    }
}

fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);
}
