fn main() {
    let mut n = 0;
    for _ in 0..=100 {
        if n == 66 {
            break
        }
        n += 1;
    }
    assert_eq!(n, 66);
}
