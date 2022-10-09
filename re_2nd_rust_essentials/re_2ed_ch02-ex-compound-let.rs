fn main() {
  let mut a = 5;
  let mut b = 6;
  let n = 7;

  let a = b = n; // b=n=7; return (), a=()
  println!("a={:?}, b={:?}, n={:?}", a, b, n); // ()77

  // no swap
  let mut c = 5;
  let mut d = 6;
  let c = d = c; // d = c = 5; return (), c = ()
  println!("c={:?}, d={:?}", c, d); // ()5
  
}

// a gets the value of the expression:  b = n;
// the value of that expression is ()
// ()77
// ()5