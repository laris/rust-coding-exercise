fn main() {
  let outer = 42;
  { // start code block
    // This variable inner only exists in this block
    let inner = 3.14;
    println!("block variable: {}", inner);
    let outer = 99; // shadows the first outer variable
    println!("block variable outer: {}", outer);
  } // end of code block
  //println!("out of block: {}", inner); 
  // error[E0425]: cannot find value `inner` in this scope
  println!("outer variable: {}", outer);
}
// block variable: 3.14
// block variable outer: 99
// outer variable: 42
