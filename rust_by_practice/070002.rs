fn main() {
    let n = 5;
    let big_n = 
        if n < 10 && n > -10 {
            println!("number is too small, increase 10 x n");
            10*n
        } else {
            println!("number is too big, decrease half n/2");
            n / 2
    };
    println!("{n} --> {big_n}");
}
