fn main() {
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk { 
        let f: Thunk = Box::new(|| println!("hi"));
        f 
    }
    
    type Result<T> = std::result::Result<T, std::io::Error>;
    
    //type Meters = u32;
    type Meters = u32;
    let x: u32 = 5;
    let y: Meters = 5;
    println!("x + y = {}", x + y);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let x = Operations::Add;
    println!("Ok");
}
