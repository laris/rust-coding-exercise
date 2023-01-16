fn main() {
    let mut n = 0;
    for _ in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }
        break;
    }
    assert_eq!(n, 66);
}
