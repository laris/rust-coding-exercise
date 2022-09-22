static mut GLOBAL_VAR: i32 = 42;

fn main() {
  // error: use of mutable static requires unsafe function or block [E0133]
  unsafe { // because it is dangerous to change a global variable!
    GLOBAL_VAR = 0;
    println!("My variable global constant: {}", GLOBAL_VAR);
  }

}

// My variable global constant: 0