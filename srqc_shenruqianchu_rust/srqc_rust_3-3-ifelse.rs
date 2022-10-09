fn main() {
    func(0);
    func(1);
    func(-1);
}

fn func(n: i32) -> () {
        if n < 0 {
            println!("{} is negative", n); 
        } else if n > 0 {
            println!("{} is positive", n); 
        } else {
            println!("{} is zero", n);
        }
    }

