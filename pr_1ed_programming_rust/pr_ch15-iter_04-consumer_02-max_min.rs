fn main() {
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
}
