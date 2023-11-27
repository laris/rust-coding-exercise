fn main() {
    assert!(0.1f32 + 0.2 == 0.3);
    assert!((0.1f64 + 0.2 - 0.3).abs() < 0.001);
}
