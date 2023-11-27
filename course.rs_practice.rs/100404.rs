trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self)}
}

impl Foo for String {
    fn method(&self) -> String {format!("String: {}", *self)}
}

fn static_dispatch<T: Foo>(x: T) {
    x.method();
}

fn dynamic_dispatch(x: &dyn Foo) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
