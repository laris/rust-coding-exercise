fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;
  (boolean, integer)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(m: Matrix) -> Matrix {
  Matrix(m.0, m.2, m.1, m.3)
}

use std::fmt;
impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
/*
    for i in 0..4 {
      write!(f, "{}", &self.i)?; 
      // error[E0609]: no field `i` on type `&Matrix`
      // ^ help: a field with a similar name exists: `0`
    }
  */
   write!(f, "( {}, {} )\n", &self.0, &self.1)?;
   write!(f, "( {}, {} )", &self.2, &self.3)
   /*
   write!(f, "( ")?;
   write!(f, "{}, ", &self.0)?;
   write!(f, "{}, ", &self.1)?;
   write!(f, "{}, ", &self.2)?;
   write!(f, "{}, ", &self.3)?;
   write!(f, " )")
   */
  }
}
/*
*/
fn main() {
  let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);

  println!("long tuple first  value: {}", long_tuple.0);
  println!("long tuple second value: {}", long_tuple.1);

  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // But long Tuples cannot be printed
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);
  // TODO ^ Uncomment the above 2 lines to see the compiler error  
  // cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`

  let pair = (1, true);
  println!("pair is {:?}", pair);
  println!("the reversed pair is {:?}", reverse(pair));

  println!("one element tuple: {:?}", (5u32,));
  println!("just an integer:   {:?}", (5u32));

  let tuple = (1, "hello", 4.5, true);
  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);
  println!("Matrix:\n{}", matrix);
  println!("Transpose:\n{}", transpose(matrix));

}
