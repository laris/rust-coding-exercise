#![allow(dead_code)]

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn main() {
  // Explicitly `use` each name so they are available without manual scoping.
  use crate::Status::{Poor, Rich};
  // Automatically `use` each name inside `Work`.
  use crate::Work::*;

  let status = Poor;
  let work = Civilian;
  match status {
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }
  match work {
    // Note again the lack of scoping.
    Civilian => println!("Civilians work!"),
    Soldier  => println!("Soldiers fight!"),
  }
}