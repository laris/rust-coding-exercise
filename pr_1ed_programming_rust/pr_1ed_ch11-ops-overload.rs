#[derive(Clone, Copy, Debug)]
struct Complex<T> {
  /// Real portion of the complex number
  re: T,
  /// Imaginary portion of the complex number
  im: T
}

/*
Category          Trait                   Operator
Unary operators   std::ops::Neg           -x
                  std::ops::Not           !x
Arithmetic        std::ops::Add           x + y
  operators       std::ops::Sub           x - y
                  std::ops::Mul           x * y
                  std::ops::Div           x / y
                  std::ops::Rem           x % y
Bitwise           std::ops::BitAnd        x & y
  operators       std::ops::BitOr         x | y
                  std::ops::BitXor        x ^ y
                  std::ops::Shl           x << y
                  std::ops::Shr           y >> y
Compound          std::ops::AddAssign     x += y
  assignment      std::ops::SubAssign     x -= y
  arithmetic      std::ops::MulAssign     x *= y
  operators       std::ops::DivAssign     x /= y
                  std::ops::RemAssign     x %= y
Compound          std::ops::BitAndAssign  x &= y
  assignment      std::ops::BitOrAssign   x |= y
  bitwise         std::ops::BitXorAssign  x ^= y
  operators       std::ops::ShlAssign     x <<= y
                  std::ops::ShrAssign     x >>= y
Comparison        std::cmp::PartialEq     x == y, x != y
                  std::cmp::PartialOrd    x < y, x <= y, x > y, x >= y
Indexing          std::ops::Index         x[y], &x[y]
                  std::ops::IndexMut      x[y] = z, &mut x[y]
*/

use std::ops::Add;

assert_eq!(4.125_f32.add(5.75), 9.875);
assert_eq!(10.add(20), 10 + 20);

trait Add<RHS=Self> {
  type Output;
  fn add(self, rhs: RHS) -> Self::Output;
}

use std::ops::Add;

impl Add for Complex<i32> {
  type Output = Complex<i32>;
  fn add(self, rhs: Self) -> Self {
    Complex { re: self.re + rhs.re, im: self.im + rhs.im }
  }
}

impl<T> Add for Complex<T>
  where T: Add<Output=T>
{
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Complex { re: self.re + rhs.re, im: self.im + rhs.im }
  }
}

use std::ops::Add;
impl<L, R, O> Add<Complex<R>> for Complex<L>
  where L: Add<R, Output=O>
{
  type Output = Complex<O>;
  fn add(self, rhs: Complex<R>) -> Self::Output {
    Complex { re: self.re + rhs.re, im: Self.im + rhs.im }
  }
}

// Unary operators
trait Neg {
  type Output;
  fn neg(self) -> Self::Output;
}

trait Not {
  type Output;
  fn not(self) -> Self::Output;
}

use std::ops::Neg;
impl<T, O> Neg for Complex<T>
  where T: Neg<Output=O>
{
  type Output = Complex<O>;
  fn neg(self) -> Complex<O> {
    Complex { re: -self.re, im: -self.im }
  }
}

// Binary operators
trait BitXor<RHS=Self> {
  type Output;
  fn bitxor(self, rhs: RHS) -> Self::Output;
}

// Compound assignment operators
trait AddAssign<RHS=Self> {
  fn add_assign(&mut self, RHS);
}

use std::ops::AddAssign;
impl<T> AddAssign for Complex<T>
  where T: AddAssign<T>
{
  fn add_assign(&mut self, rhs: Complex<T>) {
    self.re += rhs.re;
    self.im += rhs.im;
  }
}

// Equality tests
assert_eq!(x == y, x.eq(&y));
assert_eq!(x != y, x.ne(&y));

trait PartialEq<Rhs: ?Sized = Self> {
  fn eq(&self, other: &Rhs) -> bool;
  fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

impl<T: PartialEq> PartialEq for Complex<T> {
  fn eq(&self, other: &Complex<T>) -> bool {
    self.re == other.re && self.im == other.im
  }
}

let x = Complex { re: 5, im: 2 };
let y = Complex { re: 2, im: 5 };
assert_eq!(x*y, Complex { re: 0, im: 29 });

#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> { }

let s = "d\x6fv\x65t\x61i\x6c".to_string();
let t = "\x64o\x76e\x74a\x69l".to_string(); assert!(s == t); 
// s and t are only borrowed...
// ... so they still have their values here.
assert_eq!(format!("{} {}", s, t), "dovetail dovetail");

assert!("ungula" != "ungulate"); 
assert!("ungula".ne("ungulate"));

// Rust’s f32 and f64 are IEEE standard floating-point values. According to that standard, expressions like 0.0/0.0 and others with no appropriate value must produce special not-a-number values, usually referred to as NaN values

assert!(f64::is_nan(0.0/0.0)); 
assert_eq!(0.0/0.0 != 0.0/0.0, true);
assert_eq!(0.0/0.0 == 0.0/0.0, false); 
assert_eq!(0.0/0.0 < 0.0/0.0, false); 
assert_eq!(0.0/0.0 > 0.0/0.0, false); 
assert_eq!(0.0/0.0 <= 0.0/0.0, false); 
assert_eq!(0.0/0.0 >= 0.0/0.0, false);

trait Eq: PartialEq<Self> { }

impl<T: Eq> Eq for Complex<T> { }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Complex<T> {
  //...
}

// Ordered comparisions
trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
  where Rhs: ?Sized 
{
  fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
  fn lt(&self, other: &Rhs) -> bool { }
  fn le(&self, other: &Rhs) -> bool { }
  fn gt(&self, other: &Rhs) -> bool { }
  fn ge(&self, other: &Rhs) -> bool { }
}

enum Ordering {
  Less,       // self < other
  Equal,      // self == other
  Greater,    // self > other
}

trait Ord: Eq + PartialOrd<Self> {
  fn cmp(&self, other: &Self) -> Ordering;
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
  lower: T, // inclusive
  upper: T, // exclusive
}

use std::cmp::{Ordering, PartialOrd};
impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
  fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
    if self == other { Some(Ordering::Equal) }
    else if self.lower >= other.upper { Some(Ordering::Greater) }
    else if self.upper <= other.lower { Some(Ordering::Less) }
    else { None }
  }
}

assert!(Interval { lower: 10, upper: 20 } < Interval { lower: 20, upper: 40 });
assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

// Overlapping intervals aren't ordered with respect to each other.
let left = Interval { lower: 10, upper: 30 }; 
let right = Interval { lower: 20, upper: 40 }; assert!(!(left < right));
assert!(!(left >= right));

// index and IndexMut
trait Index<Idx> {
  type Output: ?Sized;
  fn index(&self, index: Idx) -> &Self::Output;
}

trait IndexMut<Idx>: Index<Idx> {
  fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

a[i..j] = *a.index(std::ops::Range { start: i, end: j });

use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("十", 10); 
m.insert("百", 100); 
m.insert("千", 1000);
m.insert("万", 1_0000); 
m.insert("億", 1_0000_0000);
assert_eq!(m["十"], 10); 
assert_eq!(m["千"], 1000);

use std::ops::Index; 
assert_eq!(*m.index("十"), 10); 
assert_eq!(*m.index("千"), 1000);

let mut desserts = vec!["Howalon".to_string(),
                        "Soan papdi".to_string()];
desserts[0].push_str(" (fictional)");
desserts[1].push_str(" (real)");

use std::ops::IndexMut;
(*desserts.index_mut(0)).push_str(" (fictional)"); 
(*desserts.index_mut(1)).push_str(" (real)");

pixels[row * bounds.0 + column] = ...;
image[row][column] = ...;

struct Image<P> {
  width: usize,
  pixels: Vec<P>
}

impl<P: Default + Copy> Image<P> {
  /// create a new image of the given size
  fn new(width: usize, height: usize) -> Image<P> {
    Image {
      width,
      pixels: vec![P::default(); width * height]
    }
  }
}

impl<P> std::ops::Index<usize> for Image<P> {
  type Output = [P];
  fn index(&self, row: usize) -> &[P] {
    let start = row * self.width;
    &self.pixels[start .. start + self.width]
  }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
  fn index_mut(&mut self, row: usize) -> &mut [P] {
    let start = row * self.width;
    &mut self.pixels[start .. start + self.width]
  }
}

// Other operators
// the error- checking ? operator works only with Result values. 
// Similarly, the logical operators && and || are limited to Boolean values only. 
// The .. operator always creates Range values, 
// the & operator always borrows references, and 
// the = operator always moves or copies values. 
// None of them can be overloaded.




