fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;
    //|x| x + num
    move |x| x + num
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
