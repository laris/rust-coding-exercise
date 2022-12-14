use std::io;

fn main() {
  print!("Give a secret positive number: ");
  let mut buf = String::new();
  io::stdin().read_line(&mut buf)
      .ok()
      .expect("Faled to read number");
  let input_num: Result<u32, _> = buf.trim().parse();
  // alternative:
  // let input_num: buf.trim().parse::<u32>();
  // println!("Unwrap found {}", input_num.unwrap());

  match input_num {
    Ok(num) => println!("{}", num),
    Err(ex) => println!("Please input an integer number! {}", ex)
  };

  // binding the value of a match:
  // let num = match input_num {
  //      Ok(num) => num,
  //      Err(_)  => 0
  // };

  let input_num: Result<u32, _> = buf.trim().parse();
  // alternative way for destructuring the Result:
  if let Ok(val) = input_num {
    println!("Matched {:?}!", val);
  } else {
    println!("No match!");
  }

  // while let Ok(val) = input_num {
  //   println!("Matched {:?}!", val);
  //   if val == 42 { break }
  // }

}
// Give a positive secret number: 42
// 42
// Matched 42!