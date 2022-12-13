fn main() {
    // count()
    use std::io::prelude::*;
    let stdin = std::io::stdin();
    //println!("{}", stdin.lock().lines().count());
    // how to read control-c and then print count result?

    // sum()
    fn triangle(n: u64) -> u64 { (1..=n).sum() }
    assert_eq!(triangle(10), 55); // 1+2+3+4+5+6+7+8+9+10=
    // product()
    fn factorial(n: u64) -> u64 { (1..=n).product() }
    assert_eq!((factorial(5)), 120); // 5!= 5*4*3*2*1 = 20*6 = 120
}
