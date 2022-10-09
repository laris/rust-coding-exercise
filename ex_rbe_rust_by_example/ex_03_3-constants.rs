// const an unchangeable value, the common case
// static a possibly mut able variable with 'static lifetime
//         the static lifetime is inferred and dose not have to specified
//         accessing or modifying a mutable static var is unsafe

static LANGUAGE: &str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main() {
  let n = 16;
  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

  // Error! Cannot modify a `const`.
  //THRESHOLD = 5;
  // FIXME ^ Comment out this line
}