/*
pub trai Iterator {
  type Item;
  fn any<F>(&mut self, f: F) -> bool where
      F: FnMut(Self::Item) -> bool {}
}
*/

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  println!("2 is vec1: {}", vec1.iter()     .any(|&x| x == 2));
  println!("2 is vec2: {}", vec2.into_iter().any(| x| x == 2));
 
  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];
  println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
  println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

}