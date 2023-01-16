fn main() {
    let x: i32 = 10;
    let y: i32 = 10;
    {
        let y: i32 = 5;
        println!("x value: {}, y value: {}", x, y);
    }
    println!("x value: {}, y value: {}", x, y);
}
