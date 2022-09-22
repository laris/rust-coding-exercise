fn main() {
  let reference = &4;

  match reference {
    &val => println!("Got a value vai destructuring: {:?}", val),
  }

  match *reference {
    val => println!("Got a value via dereferencing: {:?}", val),
  }

  let    _not_a_reference = 3;
  let ref _is_a_reference = 3;

  let value = 5;
  let mut mut_value = 6;

  match mut_value {
    ref mut m => {
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m);
    },
  }

}