use thiserror::Error;

#[derive(Debug, Copy, Clone)]
struct NeverZero(i32);

#[derive(Debug, Error)]
enum NeverZeroErr {
    #[error("Value cannot be Zero, input: {0}")]
    ZeroErr(i32),
}

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, NeverZeroErr> {
        if i == 0 {
            Err(NeverZeroErr::ZeroErr(i))
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a / b
}

fn do_divide(num: i32) {
    match NeverZero::new(num) {
        Ok(nz) => println!("{:?}", divide(10, nz)),
        //Err(e) => println!("{:?}", e), // Debug
        Err(e) => println!("{}", e),     // Display
    }
}

fn main() {
    do_divide(0);
    do_divide(5);
}
