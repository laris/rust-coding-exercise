type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64; // warning: type `u64_t` should have an upper camel case name
// ^^^^^ help: convert the identifier to upper camel case: `U64T`

fn main() {
  let nanoseconds: NanoSecond = 5 as u64_t;
  let inches: Inch = 2 as u64_t;

  println!("{} nanoseconds + {} inches = {} unit?",
           nanoseconds,
           inches,
           nanoseconds + inches);
}
