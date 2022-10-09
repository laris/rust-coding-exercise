// this is not work now

use std::io;
use futures::executor::block_on;

fn read_from_input() -> u64 {
    let mut input = String::new();
    println!("Please input an integer: ");
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("Input value: {}", input.trim());
            println!("Input byte {}", n);
        }
        Err(error) => println!("error: {}", error),
    }
    input.trim().parse::<u64>().unwrap()
}
async fn async_fn(x: u64) -> u64 {
    let msg = read_from_input().await;
    let result = calculate(msg, msg).await;
    result
}
async fn calculate(msg: u64, x: u64) {
    println!("async_calculate result: {}", msg + x);
    msg + x
}
fn main() {
    //println!("{}", read_from_input());
    println!("{}", async_fn(0));
    block_on(async_fn(0));
}
