use std::ops::Mul;

fn multiply<T: Mul<Output=T>>(lhs: T, rhs: T) -> T {
    lhs * rhs
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
}
