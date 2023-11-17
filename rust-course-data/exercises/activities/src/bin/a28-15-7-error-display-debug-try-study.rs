

#[derive(Debug)]
enum SomeErr {
    ErrType1,
    ErrType2,
}

fn main() {
    let e1 = SomeErr::ErrType1;
    let result1: Result<_, SomeErr> = Err::<(), SomeErr>(e1);
    let r2 = Err::<(), SomeErr>(result1.unwrap_err());
    //println!("{:?}", result1);
    println!("{:?}", r2);
}
