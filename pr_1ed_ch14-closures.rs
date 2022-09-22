// sort a vec of int is easy
integers.sort();

// sort data (struct) is hard
struct City {
  name: String,
  population: i64,
  country: String,
  // ...
}

fn sort_cities(cities: &mut Vec<City>) {
  cities.sort(); // error: how do you want them sorted?
}
// Rust complains that City does not implement std::cmp::Ord
/// Helper function for sorting cities by population
fn city_population_descending(city: &City) -> i64 {
  -city.population
}
fn sort_cities(cities: &mut Vec<City>) {
  cities.sort_by_key(city_population_descending); // ok
}
// The sort_by_key method takes this key-function as a parameter.

// closure version
// Rust infers the argument type and return type from how the closure is used.
fn sort_cities(cities: &mut Vec<City>) {
  cities.sort_by_key(|city| -city.population);
}

// closure for std features
/* 
   iterator map filter
   threading api thread::spawn
   method conditionally to compute a default value
    like or_insert_with method of HashMap entries
*/
// capturing variables
/// Sort by any of several diff statistics.
fn sort_by_statistic(citie: &mut Vec<City>, stat: Statistic) {
  cities.sort_by_key(|city| -city.get_statistic(stat));
}

// Closures That Borrow
fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
  cities.sort_by_key(|city| -city.get_statistic(stat));
}

// Closures that steal
use std::thread;
fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) 
   -> thread::JoinHandle<Vec<City>>
{
  let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };
  thread::spawn(move || {
    cities.sort_by_key(key_fn);
    cities
  })
}

// Function and closure types
// fn(&City) -> i64
let my_key_fn: fn(&City) -> i64 =
  if user.prefs.by_population {
    city_population_descending
  } else {
    city_monster_attack_risk_descending
  };

cities.sort_by_key(my_key_fn);

/// Given a list of cities and a test function,
/// return how many cities pass the test
fn count_selected_cities(cities: &Vec<City>,
                          test_fn: fn(&City) -> bool) -> usize
{
  let mut count = 0;
  for city in cities {
    if test_fn(city) {
      count += 1;
    }
  }
  count
}
/// An example of a test function. Note that the type of
/// this function is `fn(&City) -> bool`, the same as
/// the `test_fn` argument to `count_selected_cities`.
fn has_monster_attacks(city: &City) -> bool {
  city.monster_attack_risk > 0.0
}

// How many cities are at risk for monster attack?
let n = count_selected_cities(&my_cities, has_monster_attacks);


let limit = preferences.acceptable_monster_risk(); let n = count_selected_cities(
  &my_cities,
  |city| city.monster_attack_risk > limit); // error: type mismatch

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize where F: Fn(&City) -> bool
  {
  let mut count = 0; for city in cities {
  if test_fn(city) { count += 1;
  } }
  count
  }
  

  fn(&City) -> bool // fn type (functions only)
  Fn(&City) -> bool // Fn trait (both functions and closures)

  count_selected_cities( &my_cities, has_monster_attacks); // ok
  count_selected_cities(
  &my_cities,
  |city| city.monster_attack_risk > limit); // also ok

// every closure you write has its own type, because a closure may contain data: values either borrowed or stolen from enclosing scopes.
// every closure has an ad hoc type created by the compiler, large enough to hold that data.
// No two closures have exactly the same type. But every closure implements a Fn trait

// Closure Performance
let food = "tacos";
let weather = Weather::Tornadoes;
// a, refer to variables
     |city| city.eats(food) && city.has(weather);
// b, contain values
move |city| city.eats(food) && city.has(weather);
// c, struct empty, no take any memory
     |city| city.eats("crawfish");

// Closures and Safety
// Closures that kill
let my_str = "hello".to_string();
let f = || drop(my_str);
f(); // ok
f(); // error: use of moved value

// FnOnce
fn call_twice<F>(closure: F) where F: Fn() {
  closure();
  closure();
}

let my_str = "hello".to_string();
let f = || drop(my_str);
call_twice(f);
// error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`

// f They implement a less powerful trait, FnOnce, the trait of closures that can be called once.
// Pseudocode for `Fn` and `FnOnce` traits with no arguments
trait Fn() -> R {
  fn call(&self) -> R; // closure.call() takeself by reference, closre not moved
}

trait FnOnce() -> R {
  fn call_once(self) -> R; // closure.call_once() take self by value, is used up
}

let dict = product_glossary();
let debug_dump_dict = || {
  for (key, value) in dict { // oops!
    println!("{:?} - {:?}", key, value);
  }
};
// error[E0382]: use of moved value: `debug_dump_dict`
//  looping over &dict rather than plain dict, to access the values by reference:
let debug_dump_dict = || {
  for (key, value) in &dict { //does not use up dict
    println!("{:?} - {:?}", key, value);
  }
};

// FnMut
// Pseudocode for `Fn`, `FnMut` and `FnOnce` traits
trait Fn() -> R {
  fn call(&self) -> R;
}

trait FnMut() -> R {
  fn call_mut(&mut self) -> R;
}

trait FnOnce() -> R {
  fn call_once(self) -> R;
}

let mut i = 0;
let incr = || {
  i += 1; // incr borrows a mut reference to i
  println!("Ding! i is now: {}", i);
};
call_twice(incr);

// Fn is the family of closures and functions that
// you can call multiple times without restriction.
// highest category also includes all fn funtions
  |arg| arg + 1
  |arg| v.contains(arg)

// FnMut is the family of closures that can be called multiple times
// if the closure itself is declared mut.
  |arg| v.push(arg)
// FnOnce is the family of closures that can be called once
// if the caller owns the closure.
  || drop(v)
// Fn() is a subtrait of FnMut(), which is a subtrait of FnOnce()
// Fn the most exclusive and most powerful category

// fix the incr
fn call_twice<F>(mut closure: F) where F: FnMut() {
  closure();
  closure();
}

let mut i = 0;
call_twice(|| i+=1); //ok!
assert_eq!(i, 2);

// Callbacks
fn main() {
  let mut router = Router::new();
  router.get("/", get_from, "root");
  router.get("/", |_: &mut Request| {
                    Ok(get_form_response())
                  }, "root");
  router.post("/gcd", post_gcd, "gcd");
  router.post("/gcd", |request: &mut Request| {
                        let numbers = get_numbers(request)?;
                        Ok(get_gcd_response(numbers))
                      }, "gcd");
  println!("Serving on http://localhost:3000...");
  Iron::new(router).http("localhost:3000").unwrap();
}

struct Request {
  method: String,
  url:    String,
  headers: HashMap<String, String>,
  body:   Vec<u8>
}
struct Response {
  code: u32,
  headers: HashMap<String, String>,
  body: Vec<u8>
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
  routes: HashMap<String, C>
}

impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
  /// Create an empty router
  fn new() -> BasicRouter<C> {
    BasicRouter { routes: HashMap::new() }
  }
  /// Add a route to the router
  fn add_route(&mut self, url: &str, callback: C) {
    self.routes.insert(url.to_string(), callback);
  }
}

// This router works fine as long as we only add one route to it:
let mut router = BasicRouter::new();
router.add_route("/",    |_|   get_form_response());
router.add_route("/gcd", |req| get_gcd_response(req));
// error[E0308]: mismatched types
// note: no two closures, even if identical, have the same type
// support a variety of types, we need to use boxes and trait objects.
type BoxedCallback = Box<Fn(&Request) -> Response>;

struct BasicRouter {
  routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
  router.add_route("/gcd", |req| get_gcd_response(req));
  // Create an empty router.
  fn new() -> BasicRouter {
  BasicRouter { routes: HashMap::new() }
  }
  // Add a route to the router.
  fn add_route<C>(&mut self, url: &str, callback: C)
    where C: Fn(&Request) -> Response + 'static
  {
  self.routes.insert(url.to_string(), Box::new(callback));
  } 
}

// add_route: a particular Fn trait, and the 'static lifetime. Rust makes us add this 'static bound. Without it, the call to Box::new(callback) would be an error, because it’s not safe to store a closure if it contains borrowed references to variables that are about to go out of scope.

impl BasicRouter {
  fn handle_request(&self, request: &Request) -> Response {
    match self.routes.get(&request.url) {
      None => not_found_response(),
      Some(callback) => callback(request)
    }
  }
}

// The Iterator and IntroIterator traits
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  //... many default methods
}

trait IntoIterator where Self::IntoIter::Item == Self::Item {
  type Item;
  type IntoIter: Iterator;
  fn into_iter(self) -> Self::IntoIter;
}

println!("There's:");
let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
for element in &v {
  println!("{}", element);
}

let mut iterator = (&v).into_iter();
while let Some(element) = iterator.next() {
  println!("{}", element);
}

// Creating Iterators
// iter and iter_mut Methods
let v = vec![4, 20, 12, 8, 6];
let mut iterator = v.iter(); 
assert_eq!(iterator.next(), Some(&4)); 
assert_eq!(iterator.next(), Some(&20)); 
assert_eq!(iterator.next(), Some(&12)); 
assert_eq!(iterator.next(), Some(&8)); 
assert_eq!(iterator.next(), Some(&6)); 
assert_eq!(iterator.next(), None);

use std::ffi::OsStr;
use std::path::Path;

let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
let mut iterator = path.iter();
assert_eq!(iterator.next(), Some(OsStr::new("C:")));
assert_eq!(iterator.next(), Some(OsStr::new("C:"))); 
assert_eq!(iterator.next(), Some(OsStr::new("Users"))); 
assert_eq!(iterator.next(), Some(OsStr::new("JimB")));

// IntoIterator Implementations
// You should usually use HashSet, but its iteration order is
// nondeterministic, so BTreeSet works better in examples
use std::collections::BTreeSet;
let mut favorites = BTreeSet::new();
favorites.insert("Lucky in the Sky With Diamonds".to_string());
favorites.insert("Liebesträume No. 3".to_string());

let mut it = favorites.into_iter();
assert_eq!(it.next(), Some("Liebesträume No. 3".to_string())); 
assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string())); 
assert_eq!(it.next(), None);

for element in     &collection { } // shared ref
for element in &mut collection { } // mutable ref
for element in      collection { } // pass value

// HashSet, BTreeSet and BinaryHeap don’t implement IntoIterator on mutable references, since modifying their elements would probably violate the type’s invariants

use std::fmt::Debug;
fn dump<T, U>(t: T)
  where T: IntoIterator<Item=U>,
{
  for u in t {
    println!("{:?}", u);
  }
}

// drain methods
use std::iter::FromIterator;

let mut outer = "Earth".to_string();
let inner = String::from_iter(outer.drain(1..4));

assert_eq!(outer, "Eh");
assert_eq!(inner, "art");

// Iterator Adapters
// map and filter
// The Iterator trait’s map adapter lets you transform an iterator by applying a closure to its items. 
// The filter adapter lets you filter out items from an iterator, using a closure to decide which to keep and which to drop.
let text = " ponies \n giraffes\niguanas \nsquid".to_string();
let v: Vec<&str> = text.lines()
                       .map(str::trim)
                       .filter(|s| *s != "iguanas") // remove it
                       .collect();
assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
assert_eq!(v, ["ponies", "giraffes", "squid"]);

fn map<B, F>(self, f: F) -> some Iterator<Item=B>
   where Self: Sized,
            F: FnMut(Self::Item) -> B;

fn filter<P>(self, predicate: P) -> some Iterator<Item=Self::Item>
   where Self: Sized,
         P: FnMut(&self::Item) -> bool;

for line in text.lines() {
  let line = line.trim();
  if line != "iguanas" {
    v.push(line);
  }
}

// filter_map and flat_map
fn filter_map<B, F>(self, f: F) -> some Iterator<Item=B>
   where Self: Sized,
            F: FnMut(Self::Item) -> Option<B>;
// when closure return None, item dropped from iteration

// scan string for whitespace-separated words
// process number and drop the other words
use std::str::FromStr;
let text = "1\nfrond .25 289\n3.1415 estuary\n";
for number in text.split_whitespace()
                  .filter_map(|w| f64::from_str(w).ok()) {
                    println!("{:4.2}", number.sqrt());
                  }

text.split_whitespace()
    .map(|w| f64::from_str(w))
    .filter(|r| r.is_ok())
    .map(|r| r.unwrap()) ;

// flat_map iterator produce the concatenation of the sequences the closure returns
fn flat_map<U, F>(self, f: F) -> some Iterator<Item=U::Item>
   where F: FnMut(Self::Item) -> U,
         U: IntoIterator;

use std::collections::HashMap;
let mut major_cities = HashMap::new(); 
major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
major_cities.insert("The United States", vec!["Portland", "Nashville"])
major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
let countries = ["Japan", "Brazil", "Kenya"];
for &city in countries.iter().flat_map(|country| &major_cities[country]) { 
  println!("{}", city);
}

// scan
let iter = (0..10)
  .scan(0, |sum, item| {
    *sum += item;
    if *sum > 10 {
      None
    } else {
      Some(item * item)
    }
  });

assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);

// take and take_while
fn take(self, n: usize) -> some Iterator<Item=Self::Item>
   where Self: Sized;
fn take_while<P>(self, predicate: P) -> some Iterator<Item=Self::Item>
   where Self: Sized, P: FnMut(&Self::Item) -> bool;

let message = "To: jimb\r\n\
   From: superego <editor@oreilly.com>\r\n\
   \r\n\
   Did you get any writing done today?\r\n\
   When will you stop wasting time plotting fractals?\r\n";
for header in message.lines().take_while(|l| !l.is_empty()) { 
  println!("{}" , header);
}

// skip and skip_while
fn skip(self, n: usize) -> some Iterator<Item=Self::Item>
   where Self: Sized;
fn skip_while<P>(self, predicate: P) -> some Iterator<Item=Self:Item>
   where Self: Sized,
            P: FnMut(&Self::Item) -> bool;

for arg in std::env::args().skip(1) {
  //...
}

for body in message.lines()
  .skip_while(|l| !l.is_empty())
  .skip(1) {
    println!("{}", body);
  }

// peekable
fn peekable(self) -> std::iter::Peekable<Self>
   where Self: Sized;

use std::iter::Peekable;
fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
   where I: Iterator<Item=char>
{
  let mut n = 0;
  loop {
    match tokens.peek() {
      Some(r) if r.is_digit(10) => {
        n = n * 10 + r.to_digit(10).unwrap();
      }
      _ => return n
    }
    tokens.next();
  }
}

let mut chars = "226153980,1766319049".chars().peekable(); 
assert_eq!(parse_number(&mut chars), 226153980);
// Look, `parse_number` didn't consume the comma! So we will. 
assert_eq!(chars.next(), Some(',')); 
assert_eq!(parse_number(&mut chars), 1766319049); 
assert_eq!(chars.next(), None);

// fuse
// The fuse adapter takes any iterator and turns into one that will definitely continue to return None once it has done so the first time:
struct Flaky(bool);

impl Iterator for Flaky {
  type Item = &'static str;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0 {
      self.0 = false;
      Some("totally the last item")
    } else {
      self.0 = true; // D'oh!
      None
    }
  }
}

let mut flaky = Flaky(true);
assert_eq!(flaky.next(), Some("totally the last item")); 
assert_eq!(flaky.next(), None); 
assert_eq!(flaky.next(), Some("totally the last item"));

let mut not_flaky = Flaky(true).fuse(); 
assert_eq!(not_flaky.next(), Some("totally the last item")); 
assert_eq!(not_flaky.next(), None); 
assert_eq!(not_flaky.next(), None);

// Reversible Iterators and rev
// std::iter::DoubleEndedIterator
trait DoubleEndedIterator: Iterator {
  fn next_back(&mut self) -> Option<Self::Item>;
}

use std::iter::DoubleEndedIterator;
let bee_parts = ["head", "thorax", "abdomen"];
let mut iter = bee_parts.iter();
assert_eq!(iter.next(), Some(&"head")); 
assert_eq!(iter.next_back(), Some(&"abdomen")); 
assert_eq!(iter.next(), Some(&"thorax"));
assert_eq!(iter.next_back(), None); 
assert_eq!(iter.next(), None);

// rev adapter
fn rev(self) -> some Iterator<Item=Self>
   where Self: Sized + DoubleEndedIterator;

let meals = ["breakfast", "lunch", "dinner"];
let mut iter = meals.iter().rev(); 
assert_eq!(iter.next(), Some(&"dinner")); 
assert_eq!(iter.next(), Some(&"lunch")); 
assert_eq!(iter.next(), Some(&"breakfast")); 
assert_eq!(iter.next(), None);

// inspect
let upper_case: String = "große".chars() 
  .inspect(|c| println!("before: {:?}", c)) 
  .flat_map(|c| c.to_uppercase()) 
  .inspect(|c| println!(" after: {:?}", c)) 
  .collect();
assert_eq!(upper_case, "GROSSE");

// chain
fn chain<U>(self, other: U) -> some Iterator<Item=Self::Item>
   where Self: Sized, U: IntoIterator<Item=Self::Item>;

let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect(); 
assert_eq!(v, [1, 2, 3, 20, 30, 40]);

let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
assert_eq!(v, [40, 30, 20, 3, 2, 1]);

// enumerate
let mut pixels = vec![0; columns * rows];
let threads = 8;
let band_rows = rows / threads + 1;
//...
let bands: Vec<&mut [u8]> = pixels.chunks_mut(band_rows * columns).collect();

for (i, band) in bands.into_iter().enumerate() {
  let top = band_rows * i;
  // start a thread to render rows `top..top + band_rows`
}

// zip
let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

use std::iter::repeat;
let endings = vec!["once", "twice", "chicken soup with rice"]; 
let rhyme: Vec<_> = repeat("going")
  .zip(endings)
  .collect();
assert_eq!(rhyme, vec![ ("going", "once"),
                        ("going", "twice"),
                        ("going", "chicken soup with rice")]);


// by_ref
let message = "To: jimb\r\n\ 
                From: id\r\n\
                \r\n\
                Oooooh, donuts!!\r\n";
let mut lines = message.lines();
println!("Headers:");
for header in lines.by_ref().take_while(|l| !l.is_empty()) {
  println!("{}" , header); 
}
println!("\nBody:"); 
for body in lines {
  println!("{}" , body); 
}

impl<'a, I: Iterator + ?Sized> Iterator for &'a mut | {
  type Item = I::Item;
  fn next(&mut self) -> Option<I::Item> {
    (**self).next()
  }
  fn size_hint(&self) -> (usize, Option<usize>) {
    (**self).size_hint()
  }
}

// cloned
let a = ['1', '2', '3', '∞'];
assert_eq!(a.iter().next(), Some(&'1')); 
assert_eq!(a.iter().cloned().next(), Some('1'));

// cycle
let dirs = ["North", "East", "South", "West"]; 
let mut spin = dirs.iter().cycle(); 
assert_eq!(spin.next(), Some(&"North")); 
assert_eq!(spin.next(), Some(&"East")); 
assert_eq!(spin.next(), Some(&"South")); 
assert_eq!(spin.next(), Some(&"West")); 
assert_eq!(spin.next(), Some(&"North")); 
assert_eq!(spin.next(), Some(&"East"));

use std::iter::{once, repeat};
let fizzes = repeat("").take(2).chain(once("fizz")).cycle(); 
let buzzes = repeat("").take(4).chain(once("buzz")).cycle(); 
let fizzes_buzzes = fizzes.zip(buzzes);
let fizz_buzz = (1..100).zip(fizzes_buzzes) 
  .map(|tuple|
    match tuple {
      (i, ("", "")) => i.to_string(),
      (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
    });
for line in fizz_buzz { 
  println!("{}", line);
}



