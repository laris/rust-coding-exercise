fn main() {
  let energy = 5; // value 5 is bound to variable energy

  // spliting declaration and initialization:
  let energy2;
  energy2 = 5;

  let _energy = 5; // no warning unused variable

  let energy = 5usize; // energy is now an unsigned integer

  let copy_energy = energy;
  println!("Your energy is {}", energy); // Your energy is 5

  let level_title = "Level 1"; // &string type
  let dead = false; // bool
  let magic_number = 3.14_f32; // fp32
  // an _ can be used to separate the digits 
  // from the type to improve readability:
  let energy = "Abundant"; // a new energy variable, drop previous one
  let empty = (); // the value of the unit type ()
  
  // changing values:
  // energy = 25; // error: re-assignment of immutable variable `energy`
  let mut fuel = 34; // mutable variable
  fuel = 60;

  let n; 
  // error: type annotations needed,
  // consider giving `energy2` a type, cannot infer type for `_
  n = -2;

  let x = 42_u8;
  let magic_number = 3.14_f64;
  // fn2() // cannot find function `fn2` in this scope 
  // - not found in this scope
}

// 3.14
// Monster Attack
// 3.141593
