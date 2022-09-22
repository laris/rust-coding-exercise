// #![feature(box_syntax)] // error: unstable feature in stable releases

fn main() {
  let health = 32;
  let mut game = "Space Invaders";
  println!("address of health-value: {:p}", &health); // prints 0x23fba4
  println!("address of game-value: {:p}", &game); // prints 0x23fb90
  println!("game-value: {}", game); // prints "Space Invaders"
  println!("game: {}", &game); // prints "Sapce Invaders"

  let game2 = &game;
  println!("{:p}", game2); // prints 0x23fb90 same with &game
  println!("{}",  *game2); // &str
  println!("{}",   game2); // &str

  // let x: &i64;
  // println!("{:?}", x); // error: use of possibly uninitialized variable: `x`
  let x: &i64 = &100;
  println!("{:p}", &100);
  println!("{:p}", x);
  println!("{:?}", x);

  let rx: *mut i64 = 0x100 as *mut i64;
  println!("{:p}", rx);
  println!("{:?}", rx);
  println!("{:?}", &rx);

  // health = 33; // error: re-assignment of immutable variable `health`
  let y = &health;
  println!("{}",  *y); // 32
  println!("{:}", *y); // 32

  // references to an immutable variable:
  let tricks = 10;
  // let ref_tricks = &mut tricks; 
  // error: cannot borrow immutable local variable `tricks` as mutable

  // references to a mutable variable:
  let mut score = 0;
  let ref_score = &score;
  // *ref_score = 5; // cannot assign to immutable borrowed content *ref_score

  let mut score = 0;
  let mref_score = &mut score;
  //let mref_score2 = &mut score;
  // error: cannot borrow `score` as mutable more than once at a time
     *mref_score = 5;
  println!("score now = {}", score); // use this or below, not both
  //println!("mref_score ref to score now = {}", mref_score);
  //println!("mref_score ref to score now = {}", *mref_score);

  // boxing values onto the heap:
  let x = Box::new(5_i32);
  // let y = box 6; // error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
  // or use feature gate #![feature(box_syntax)]
  // let x = box 5i32; 
  
}

// address of health-value: 0x23fb04
// address of game-value: 0x23faf0
// game-value: Space Invaders
// game: Space Invaders
// 0x23faf0
// Space Invaders
// Space Invaders
// 32
