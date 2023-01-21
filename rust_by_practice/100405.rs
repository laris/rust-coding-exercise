trait MyTrait {
    //fn f(&self) -> Self;
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    //fn f(&self) -> Self { 42 }
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}
impl MyTrait for String {
    //fn f(&self) -> Self { self.clone() }
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    my_function(Box::new(13u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
