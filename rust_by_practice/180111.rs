fn factory(i: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    if i > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn main() {
    let answer = factory(2);
    println!("{}", answer(5));
    let answer = factory(1);
    println!("{}", answer(5));

    println!("{}", (factory(1))(5));
}
