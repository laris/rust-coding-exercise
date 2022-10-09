fn main() {
/*
  let _max: u64 = 10;
  //let op1: Option<u8> = Some(1);
  let op1: Option<u8> = None;
  //for i in 1..max {
  for i in op1 {
    print!("{:?} ", i);
  }
  println!("");
*/
/*
use std::iter::{once, repeat};
let fizzes = repeat("").take(2).chain(once("fizz")).cycle(); 
let buzzes = repeat("").take(4).chain(once("buzz")).cycle(); 
let fizzes_buzzes = fizzes.zip(buzzes);
let fizz_buzz = (1..100).zip(fizzes_buzzes) 
  .map(|tuple|
    match tuple {
      (i, ("", "")) => i.to_string(),
      (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
    });
for line in fizz_buzz { 
  println!("{}", line);
}
*/
let mut int_list = (0..10);
println!("{:?}", int_list.nth(4));
//jjprintln!("{:?}", int_list.nth(1));
//println!("{:?}", int_list.nth(2));
//println!("{:?}", int_list.nth(3));


}