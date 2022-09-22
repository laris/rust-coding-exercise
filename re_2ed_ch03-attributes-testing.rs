fn main() {
  println!("No tests are compiled, compile with rustc --test!");
  double(42);
}

pub fn double(n: i32) -> i32 {
  n * 2
}

/*
#[test]
fn arithmetic() {
  // good tests
  // assert_eq!(actual, expected);
/*
  assert_eq!(5, 2 + 3);
  assert_eq!(6, 2 + 3);
  assert_eq!(double(42), 84);
  assert!(2 + 3 == 5);
  assert!(2 + 3 == 6);
*/
  // bad tests:
/*
  if 2 + 3 == 5 {
    println!("You can calculate!");
  }
  if 2 + 3 == 6 { // this test passes as well!
    println!("You cannot calculate!");
  }
*/
}

#[test]
fn badtest() {
  assert_eq!(6, 2 + 3);
}
*/

#[test]
//#[ignore]
#[should_panic(expected = "assertion failed")]
//#[should_panic(expected = "assertion failed", expected = "6")]
//#[should_panic(expected = "6")]
fn failing_test() {
  assert!(6 == 2 + 3);
}
