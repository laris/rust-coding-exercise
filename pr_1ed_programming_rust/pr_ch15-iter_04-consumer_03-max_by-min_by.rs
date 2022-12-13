fn main() {
    use std::cmp::Ordering;
    // Compare two f64 values. Panic if given a NaN.
    fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }

    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    //panic
    //assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
}
