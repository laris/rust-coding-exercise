/*
let mut optional = Some(0);
loop {
  match optional {
    Some(i) => {
      if i > 9 {
        println!("Greater than 9, quit!");
        optional = None;
      } else {
        println!("i is {:?}, try again", i);
        optional = Some(i + 1);
      }
    },
    _ => { break; }
  }
}

fn main() {
  let mut optional = Some(0);
  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit");
      optional = None;
    } else {
      println("i is {:?}, try again", i);
      optional = Some(i + 1);
    }
  }
}

*/

// Make `optional` of type `Option<i32>`
let mut optional = Some(0);

// Repeatedly try this test.
loop {
    match optional {
        // If `optional` destructures, evaluate the block.
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ Requires 3 indentations!
        },
        // Quit the loop when the destructure fails:
        _ => { break; }
        // ^ Why should this be required? There must be a better way!
    }
}

fn main() {
  // Make `optional` of type `Option<i32>`
  let mut optional = Some(0);

  // This reads: "while `let` destructures `optional` into
  // `Some(i)`, evaluate the block (`{}`). Else `break`.
  while let Some(i) = optional {
      if i > 9 {
          println!("Greater than 9, quit!");
          optional = None;
      } else {
          println!("`i` is `{:?}`. Try again.", i);
          optional = Some(i + 1);
      }
      // ^ Less rightward drift and doesn't require
      // explicitly handling the failing case.
  }
  // ^ `if let` had additional optional `else`/`else if`
  // clauses. `while let` does not have these.
}
