trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;
  fn write_all(&mut self, buf: &[u8]) -> Result<()> {}
}

use std::io::Write;

fn say_hello(out: &mut Write) -> std::io::Result<()> {
  out.write_all(b"hello world\n")?;
  out.flush()
}

use std::fs::File;
let mut local_file = File::create("hello.txt")?;
say_hello(&mut local_file)?; // works

let mut bytes = vec![];
say_hello(&mut bytes)?; // also works
assert_eq!(bytes, b"hello world\n");

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
  if value1 <= value2 {
    value1
  } else {
    value2
  }
}

// Using traits
// the trait itself must be in scope
use std::io::Write; // must add this to fix
let mut buf: Vec<u8> = vec![];
buf.write_all(b"hello")?; // error: no method named `write_all`

// Trait objects
use std::io::Write;

let mut buf: Vec<u8> = vec![];
// no permit variables of type Write
let write: Write = buf; // error: `Write` does not have a constant size
// references are explicit
let writer: &mut Write = &mut buf; // ok
// A reference to a trait type, like writer, is called a trait object.

// Generic Functions
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
  out.write_all(b"hello world\n")?;
  out.flush()
}

fn say_hello(out: &mut Write) // plain function
fn say_hello<W: Write>(out: &mut W) // generic function, type parameter

say_hello(&mut local_file)?; // calls say_hello::<File>
say_hello::<File>(&mut local_file)?; // not necessary
say_hello(&mut bytes)?; // calls say_hello::<Vec<u8>>

// calling a generic method collect<C>() that takes no arguments
let v1 = (0 .. 1000).collect(); // error: can't infer type
let v2 = (0 .. 1000).collect::<Vec<i32>>(); // ok

use std::fmt::Debug;

fn top_ten<T: Debug>(values: &Vec<T>) { }
fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { }

// generic functions can have multiple type parameters
/// Run a query on a large, partitioned data set
/// See <http://research.google.com/archive/mapreduce.html>
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize> (
  data: &Dataset, map: M, reduce: R) -> Results
{ }

fn run_query<M, R>(data: &Dataset, map: M, reduce: R) -> Results
  where M: Mapper  + Serialize,
        R: Reducer + Serialize
{ }

// generic function have both lifetime parameters and type parameters,life first
/// Return a reference to the point in `candidates`
/// that's closest to the `target` point
fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
  where P: MeasureDistance
{

}

impl PancakeStack {
  fn push<T: Topping>(&mut self, goop: T) -> PancakeResult<()> {
    //...
  }
}

type PancakeResult<T> = Result<T, PancakeError>;

// trait objects are the right choice whenever u need a collection of values
// of mixed types, all together
trait Vegetable { }
struct Salad<V: Vegetable> {
  veggies: Vec<V>
}

struct Salad {
  veggies: Vec<Vegetable> // error: `Vegetable` does not have a constant size
}

struct Salad {
  veggies: Vec<Box<Vegetable>>
}

// defining and implementing traits
/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen
trait Visible {
  /// Render this object on the given canvas.
  fn draw(&self, canvas: &mut Canvas);
  /// return true if clicking at (x, y) should select this object
  fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
  fn draw(&self, canvas: &mut Canvas) {
    for y in self.y - self.height - 1 .. self.y {
      canvas.write_at(self.x, y, '|');
    }
    canvas.write_at(self.x, self.y, 'M');
  }
  fn hit_test(&self, x: i32, y: i32) -> bool {
    self.x == x
    && self.y - self.height - 1 <= y
    && y <= self.y
  }
}

impl Broom {
  /// Helper function used by Broom::draw() below
  fn broomstick_range(&self) -> Range<i32> {
    self.y - self.height - 1 .. self.y
  }
}

impl Visible for Broom {
  fn draw(&self, canvas: &mut Canvas) {
    for y in self.broomstick_range() {
      //...
    }
    //...
  }
}

// Default methods
/// A Writer that ignores whatever data you write to it.
pub struct Sink;

use std::io::{Write, Result};

impl Write for Sink {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    // Claim to have successfully written the whole buffer.
    Ok(buf.len())
  }
  fn flush(&mut self) -> Result<()> {
    Ok(())
  }
}

trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;

  fn write_all(&mut self, buf: &[u8]) -> Result<()> {
    let mut bytes_written = 0;
    while bytes_written < buf.len() {
      bytes_written += self.write(&buf[bytes_written..])?;
    }
    Ok(())
  }
}

// Traits and other people's types
trait IsEmoji {
  fn is_emoji(&self) -> bool;
}

/// Implement IsEmoji for the built-in character type
impl IsEmoji for char {
  fn is_emoji(&self) -> bool {
    //...
  }
}

assert_eq!('$'.is_emoji(), false);

// extension trait adds a method to all Rust writers
use std::io::{self, Write};
/// Trait for values to which you can send HTML
trait WriteHtml {
  fn write_html(&mut self, &HtmlDocuement) -> io::Result<()>;
}
/// You can write HTML to any std::io writer
impl<W: Write> WriteHtml for W {
  fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
    //...
  }
}

// serde is a serialization library
use serde::Serialize;
use serde_json;

pub fn save_configuration(config: &HashMap<String, String>) 
  -> std::io::Result<()>
{
  // Create a JSON serializer to write the data to a file
  let writer = File::create(config_filename())?;
  let mut serializer = serde_json::Serializer::new(writer);
  // The serde `.serialize()` method does the rest
  config.serialize(&mut serializer)?;
  Ok(())
}

// Self in traits
// clone trait
pub trait Clone {
  fn clone(&self) -> Self;
}

pub trait Spliceable {
  fn splice(&self, other: &Self) -> Self;
}

impl Spliceable for CherryTree {
  fn splice(&self, other: &Self) -> Self {
    //...
  }
}

impl Spliceable for Mammoth {
  fn splice(&self, other: &Self) -> Self {
    //...
  }
}

// error: the trait `Spliceable` cannot be made into an object
fn splice_anything(left: &Spliceable, right: &Spliceable) {
  let combo = left.splice(right);
  //...
}

pub trait MegaSpliceable {
  fn splice(&self, other: &MegaSpliceable) -> Box<MegaSpliceable>;
}

// Subtraits
/// Someone in the game world, either the player or some other
/// pixie, gargoyle, squirrel, ogre, etc.
trait Creature: Visible {
  fn position(&self) -> (i32, i32);
  fn facing(&self) -> Direction;
  // ...

}
// Every type that implements Creature 
// must also implement the Visible trait:
impl Visible for Broom { ...
}
impl Creature for Broom { ...
}

// static methods
// rust trait can include static methods and constructors
trait StringSet {
  /// Return a new empty set
  fn new() -> Self;
  /// Return a set that contains all the strings in `strings`
  fn from_slice(strings: &[&str]) -> Self;
  /// Find out if this set contains a particular `value`
  fn contain(&self, string: &str) -> bool;
  /// Add a string to this set
  fn add(&mut self, string: &str);
}

// Create sets of two hypothetical types that impl StringSet
let set1 = SortedStringSet::new();
let set2 = HashedStringSet::new();

/// Return the set of words in `document` that aren't in `wordlist`
fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
  let mut unknowns = S::new();
  for word in document {
    if !wordlist.contains(word) {
      unknowns.add(word);
    }
  }
  unknowns
}

trait StringSet {
  fn new() -> Self
    where Self: Sized;
  fn from_slice(strings: &[&str]) -> Self 
    where Self: Sized;
  fn contains(&self, string: &str) -> bool;
  fn add(&mut self, string: &str);
}

// fully qualified method calls
// All four of these method calls do exactly the same thing.
"hello".to_string()
str::to_string("hello")
ToString::to_string("hello")
<str as ToString>::to_string("hello") // fully qualified method call

outlaw.draw(); // error: draw on screen or draw pistol?
Visible::draw(&outlaw); // ok: draw on screen 
HasPistol::draw(&outlaw); // ok: corral

let zero = 0; // type unspecified; could be i8/u8/...
zero.abs(); // error: method `abs` not found
i64::abs(zero); // ok

let words: Vec<String> =
  line.split_whitespace() // iterator produces &str values
    .map(<str as ToString>::to_string) // ok
    .collect();

// iterator
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  //...
}

// code from the std::env standard library module
impl Iterator for Args {
  type Item = String;
  fn next(&mut self) -> Option<String> {
    //...
  }
}

/// Loop over an iterator, storing the values in a new vector
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
  let mut results = Vec::new();
  for value in iter {
    result.push(value);
  }
  results
}

/// Print out all the values produced by an iterator
fn dump<I>(iter: I) 
  where I: iterator
{
  for (index, value) in iter.enumerate() {
    println!("{}: {:?}", index, value); // error
  }
}

use std::fmt::Debug;
fn dump<I>(iter: I)
  where I: Iterator, I::Item: Debug
//where I: Iterator<Item=Debug>
{
  for (index, value) in iter.enumerate() {
    println!("{}: {:?}", index, value); // 
  }
}

fn dump<I>(iter: I)
  where I: Iterator<Item=String>
{
  //...
}

fn dump(iter: &mut Iterator<Item=String>) {
  for (index, s) in iter.enumerate() {
    println!("{}: {:?}", index, s);
  }
}

trait Pattern {
  type Match;
  fn search(&self, string: &str) -> Option<Self:Match>;
}
/// You can search a string for a particular character
impl Pattern for char {
  /// A "match" is just the location where the character was found
  type Match = usize;
  fn search(&self, string: &str) -> Option<usize> {
    //...
  }
}

// Generic Traits or how operator overload works
/// std::ops::Mul, the trait for types that support `*`
pub trait Mul(RHS) { // RHS = Right Hand Side
  /// The resulting type after applying the `*` operator
  type Output;
  /// The method for the `*` operator
  fn mul(self, rhs: RHS) -> Self::Output;
}

// Buddy traits or how rand::random() works
use rand::random;
let x = random();
let x = random::<f64>(); // a number, 0.0 <= x < 1.0
let b = random::<bool>(); // true or false

/// A random number generator
pub trait Rng {
  fn next_u32(&mut self) -> u32;
}

/// A type that can be randomly generated using an `Rng`
pub trait Rand: Sized {
  fn rand<R: Rng>(rng: &mut R) -> Self;
}

let x = f64::rand(rng);
let b = bool:rand(rng);

pub fn random<T: Rand>() -> T {
  T::rand(&mut global_rng())
}

// reverse-engineering bounds
fn dot(v1: &[i64], v2: &[i64]) -> i64 {
  let mut total = 0;
  for i in 0 .. v1.len() {
    total = total + v1[i] * v2[i];
  }
  total
}

// Rust complains about the use of + and * and the type of 0.
fn dot<N>(v1: &[N], v2: &[N]) -> N {
  let mut total: N = 0;
  for i in 0 .. v1.len() {
    total = total + v1[i] * v2[i];
  }
  total
}

use std::ops::{Add, Mul};
fn dot<N: Add + Mul + Default>(v1: &[N], v2: &[N]) -> N {
  let mut total = N::default();
  for i in 0 .. v1.len() {
    total = total + v1[i] * v2[i]; 
    // expected type parameter, found associated type
    // = note: expected type `N` found type `<N as std::ops::Mul>::Output`
  }
  total
}

fn dot<N: Add<Output=N> + Mul<Output=N> + Default>(v1: &[N], v2: &[N]) -> N
{ }
fn dot<N>(v1: &[N], v2: &[N]) -> N
  where N: Add<Output=N> + Mul<Output=N> + Default
{ }
// error[E0508]: cannot move out of type `[N]`, a non-copy array
  where N: Add<Output=N> + Mul<Output=N> + Default + Copy

// final code
use std::ops::{Add, Mul};

fn dot<N>(v1: &[N], v2: &[N]) -> N
  where N: Add<Outpu=N> + Mul<Output=N> + Default + Copy
{
  let mut total = N::default();
  for i in 0 .. v1.len() {
    total = total + v1[i] * v2[i];
  }
  total
}

#[test]
fn test_dot() {
  assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
  assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}


