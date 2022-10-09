fn main() {
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  let i = 1;
  let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` u8  in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` u32 in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` f32 in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` i32 in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` f64 in bytes: {}", std::mem::size_of_val(&f));
}