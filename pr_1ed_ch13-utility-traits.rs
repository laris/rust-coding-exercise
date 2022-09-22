// Drop
struct Appellation {
  name: String,
  nicknames: Vec<String>
}

trait Drop {
  fn drop(&mut self);
}

impl Drop for Appellation {
  fn drop(&mut self) {
    print!("Dropping {}", self.name);
    if !self.nicknames.is_empty() {
      print!(" (AKA {})", self.nicknames.join(", "));
    }
    println!("");
  }
}

fn main() {
  let mut a = Appellation { name: "Zeus".to_string(),
              nicknames: vec!["cloud collector".to_String(),
                              "king of gods".to_string()]};
  println!("before assignment");
  a = Appellation { name: "hera".to_string(), nicknames: vec![]};
  println!("at end of block");
}

let p;
{ 
  let q = Appellation { 
          name: "Cardamine hirsuta".to_string(),
          nicknames: vec!["shotweed".to_string(),
                          "bittercress".to_string()]};
  if complicated_condition() {
    p = q;
  }
}
println!("Sproing! What was that?");

struct FileDesc {
  fd: c_int,
}

impl Drop for FileDesc {
  fn drop(&mut self) {
    let _ = unsafe { libc::close(self.fd) };
  }
}

fn drop<T>(_x: T) { }

// Sized
struct RcBox<T: ?Sized> {
  ref_count: usize,
  value: T,
}

let boxed_lunch: RcBox<String> = RcBox {
  ref_count: 1,
  value: "lunch".to_string()
};

use std::fmt::Display;
let boxed_displayable: &RcBox<Display> = &boxed_lunch;

fn display(boxed: &RcBox<Display>) {
  println!("For your enjoyment: {}", &boxed.value);
}

display(&boxed_lunch);

// Clone
trait Clone: Sized {
  fn clone(&self) -> Self;
  fn clone_from(&mut self, source: &Self) {
    *self = source.clone()
  }
}

// Copy
trait Copy: Clone { }
impl Copy for MyType { }

// Deref and DerefMut
trait Deref {
  type Target: ?Sized;
  fn deref(&self) -> &Self::Target;
}

trait DerefMut: Deref {
  fn deref_mut(&mut self) -> &mut Self::Target;
}

// deref coercoin
let r = Rc<String>; // &Rc<String> coerces to &String because Rc<T> impl Deref<Target=T>
r.find('?'); // borrow r
(*r).find('?');

struct Selector<T> {
  /// Elements available in the `Selector`
  elements: Vec<T>,
  /// The index of the "current" element in `elements`.
  /// A `Selector` behaves like a pointer to the currect element.
  current: usize
}

use std::ops::{Deref, DerefMut};
impl<T> Deref for Selector<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.elements[self.current]
  }
}
impl<T> DerefMut for Selector<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.elements[self.current]
  }
}

let mut s = Selector { 
  elements: vec!['x', 'y', 'z'],
  current: 2 };
// Because `Selector` implements `Deref`, we can use the `*` operator to 
// refer to its current element.
assert_eq!(*s, 'z');
// Assert that 'z' is alphabetic, using a method of `char` directly on a 
// `Selector`, via deref coercion.
assert!(s.is_alphabetic());
// Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
*s = 'w';
assert_eq!(s.elements, ['x', 'y', 'w']);

let s = Selector {
                  elements: vec!["good", "bad", "ugly"],
                  current: 2 };
fn show_it(thing: &str) { println!("{}", thing); }
show_it(&s);
show_it(s.deref());

use std::fmt::Display;
fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }
show_it_generic(&s);
// error[E0277]: the trait bound `Selector<&str>: Display` is not satisfied
show_it_generic(&s as &str)

// Default
// Vec/String empty, number zero, Option None
trait Default {
  fn default() -> Self;
}

impl Default for String {
  fn default() -> String {
    String::new()
  }
}

use std::collections::HashSet;
let squares = [4, 9, 16, 25, 36, 49, 64];
let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
  = squares.iter().partition(|&n| n & (n-1) == 0);
assert_eq!(powers_of_two.len(), 3); // 2^2=4, 2^4=16, 2^6=64
assert_eq!(impure.len(), 4);

let (upper, lower): (String, String)
  = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
assert_eq!(upper, "GTO");
assert_eq!(lower, "reat eacher nizuka");

let params = glium::DrawParameters {
  line_width: Some(0.02),
  point_size: Some(0.02),
  ..Default::default()
};
target.draw(..., &params).unwrap();

// AsRef and AsMut
trait AsRef<T: ?Sized> {
  fn as_ref(&self) -> &T;
}
trait AsMut<T: ?Sized> {
  fn as_mut(&mut self) -> &mut T;
}

// std::fs::File::open
fn open<P: AsRef<Path>>(path: P) -> Result<File>

let dot_emacs = std::fs::File::open("homejimb/.emacs")?;

impl<'a, T, U> AsRef<U> for &'a T
  where T: AsRef<U>,
        T: ?Sized,
        U: ?Sized
{
  fn as_ref(&self) -> &U {
    (*self).as_ref()
  }
}

// Borrow and BorrowMut
trait Borrow<Borrowed: ?Sized> {
  fn borrow(&self) -> &Borrowed;
}

impl HashMap<K, V> where K: Eq + Hash
{
  fn get(&self, key: K) -> Option<&V> { }
}
// must pass a String K by value to call to get, wasteful
impl HashMap<K, V> where K: Eq + Hash
{
  fn get(&self, key: &K) -> Option<&V> { }
}
// better, have to pass the key as &String
hashtable.get(&"twenty-two".to_string());
// it allocate a String buffer on the heap and cp the text into it
// it borrow it as a &String, pass to get, then drop it

// final
impl HashMap<K, V> where K: Eq + Hash
{
  fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q>,
          Q: Eq + Hash
  { }
}

// The standard library includes a blanket implementation so that every type T can be borrowed from itself: T: Borrow<T>

// BorrowMut
trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed> {
  fn borrow_mut(&mut self) -> &mut Borrowed;
}

// From and Into
// From and Into take ownership of their argument, transform it, and then return ownership of the result back to the caller.
// std lib, every tpe T impl From<T> Into<T>
trait Into<T>: Sized {
  fn into(self) -> T;
}

trait From<T>: Sized {
  fn from(T) -> Self;
}

use std::net::Ipv4Addr;
fn ping<A>(address: A) -> std::io::Result<bool>
  where A: Into<Ipv4Addr>
{
  let ipv4_address = address.into();
}

println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141))); // pass an Ipv4Addr 
println!("{:?}", ping([66, 146, 219, 98])); // pass a [u8; 4] 
println!("{:?}", ping(0xd076eb94_u32)); // pass a u32

let addr1 = Ipv4Addr::from([66, 146, 219, 98]); 
let addr2 = Ipv4Addr::from(0xd076eb94_u32);

// ownership, conversion, original values' resource
let text = "Beautiful Soup".to_string();
let bytes: Vec<u8> = text.into();

// ToOwned
trait ToOwned {
  type Owned: Borrow<Self>;
  fn to_owned(&self) -> Self::Owned;
}

// Borrow and ToOwned at work: the humble cow
enum Cow<'a, B: ?Sized + 'a>
  where B: ToOwned
{
  Borrowed(&'a B),
  Owned(<B as ToOwned>::Owned),
}
//  if it’s Owned, it borrows a shared reference to the owned value; 
// and if it’s Borrowed, it just hands out the reference it’s holding.
use std::path::PathBuf;
use std::borrow::Cow;
// Most of the variants can be handled with fixed strings, 
// but some of them have additional data that should be included 
// in the message. You can return a Cow<'static, str>:
fn describe(error: &Error) -> Cow<'static, str> {
  match *error {
    Error::OutOfMemory => "out of memory".into(),
    Error::StackOverflow => "stack overflow".into(),
    Error::MachineOnFire => "machine on fire".into(),
    Error::Unfathomable => "machine bewildered".into(),
    Error::FileNotFound(ref path) => {
      format!("file not found: {}", path.display()).into()
    }
  }
}

println!("Disaster has struck: {}", describe(&error)); // Cow as &str

let mut log: Vec<String> = Vec::new();
//...
log.push(describe(&error).into_owned());
// Using Cow helps describe and its callers put off allocation until the moment it becomes necessary.







