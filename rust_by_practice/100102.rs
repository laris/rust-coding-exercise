fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

fn sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y
}
