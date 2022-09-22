fn main() {
  let elem = 5u8;
  let mut vec = Vec::new();
  vec.push(elem);
  // error[E0282]: type annotations needed for `std::vec::Vec<T>`
  // ^^^^^^^^ cannot infer type for type parameter `T`
  println!("{:?}", vec);

}