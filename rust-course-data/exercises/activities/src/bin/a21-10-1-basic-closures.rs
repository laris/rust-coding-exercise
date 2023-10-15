fn main() {
    fn add_fn(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add_fn(1, 2);
    let add_closure = |a: i32, b: i32| -> i32 {
        a + b
    };

    let add_closure2 = |a, b| a + b;
    let sum = add_closure2(1, 2);
}
