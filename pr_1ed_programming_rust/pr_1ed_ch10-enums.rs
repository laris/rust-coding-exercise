enum Ordering {
  Less,
  Equal,
  Greater,
}

use std::cmp::Ordering;

fn compare(n: i32, m:i32) -> Ordering {
  if n < m {
    Ordering::Less
  } else if n > m {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
}

use std::cmp::Ordering;
use std::cmp::Ordering::*; // `*` to import all children

fn compare(n:i32, m: i32) -> Ordering {
  if n < m {
    Less
  } else if n > m {
    Greater
  } else {
    Equal
  }
}


enum Pet {
  Orca,
  Giraffe,
  Dog,
}

use self::Pet::*;

enum HttpStatus{
  Ok = 200,
  NotModified = 304,
  NotFound = 404,
}

use std::mem::size_of;
assert_eq!(size_of::<Ordering>(), 1);
assert_eq!(size_of::<HttpStatus>(), 2); // 404 dosen't fit in a u8

assert_eq!(HttpStatus::Ok as i32, 200);

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
  match n {
    200 => Some(HttpStatus::Ok),
    304 => Some(HttpStatus::NotMOdified),
    404 => Some(HttpStatus::NotFound),
    _   => None
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
  Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
  /// Return the plural noun for this time unit
  fn plural(self) -> &'static str {
    match self {
      TimeUnit::Seconds => "seconds",
      TimeUnit::Minutes => "minutes",
      TimeUnit::Hours   => "hours",
      TimeUnit::Days    => "days",
      TimeUnit::Months  => "months",
      TimeUnit::Years   => "years"
    }
  }

  /// Return the singular noun for this time unit
  fn singular(self) -> &'static str {
    self.plural().trim_right_matches('s')
  }
}

/// A timestamp that has been deliberately rounded off
/// so our program says "6 months ago" instead of
/// "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
  InThePast(TimeUnit, u32),
  JustNow,
  InTheFuture(TimeUnit, u32),
}

let four_score_and_seven_year_ago =
  RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);

let three_hours_from_now = 
  RoughtTime::InTheFuture(TimeUnit::Hours, 3);

enum Shape {
  Shpere { center: Point3d, radius: f32 },
  Cuboid { corner1: Point3d, corner2: Point3d }
}

let unit_shpere = Shape::Shpere { center: ORIGIN, radius: 1.0 };

// enum can have variants of all three kinds.
enum RelationshipStatus {
  Single,
  InARelationship,
  NotFound = 404,
  ItsComplicated(Option<String>, u32),
  ItsExtremelyComplicated {
    car: DifferentialEquation,
    cdr: EarlyModernistPoem
  }
}

enum Json {
  Null,
  Boolean(bool),
  Number(f64),
  String(String),
  Array(Vec<Json>),
  Object(Box<HashMap<String, Json>>)
}


enum Option<T> {
  None,
  Some(T)
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}

// An ordered collection of `T`s.
enum BinaryTree<T> {
  Empty,
  NonEmpty(Box<TreeNode<T>>)
}

// Apart of a Binary Tree.
struct TreeNode<T> {
  element: T,
  left:   BinaryTree<T>,
  right:  BinaryTree<T>
}

use self::BinaryTree::*;
let jupiter_tree = NonEmpty(Box::new(TreeNode {
  element:"Jupiter",
  left:  Empty,
  right: Empty
}));

let mars_tree = NonEmpty(Box::new(TreeNode {
  element: "Mars",
  left:  jupiter_tree,
  right: mercury_tree
}));

let tree = NonEmpty(Box::new(TreeNode {
  element: "Sarturn",
  left: mars_tree,
  right: uranus_tree
}));

let mut tree = BinaryTree::Empty;
for planet in planets {
  tree.add(planet);
}

// Patterns
fn rough_time_to_english(rt: RoughTime) -> String {
  match rt {
    RoughTime::InThePast(units, count) => 
            format!("{} {} ago", count, units.plural()),
    RoughTime::JuswNow =>
            format!("just now"),
    RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural())
  }
}

match meadow.count_rabbits() {
  0 => {} // nothing to say
  1 => println!("A rabbit is nosing around in the clover."),
  n => println!("There are {} rabits hopping about in the meadow", n)
  // the matched value is moved/copied into new local variable n
}

let calendar = 
  match settings.get_string("calendar") {
    "gregorian" => Calendar::Gregorian,
    "chinese"   => Calendar::Chinese,
    "ethiopian" => Calendar::Ethiopian,
    other       => return parse_error("calendar", other)
  };

let caption = 
  match photo.tagged_pet() {
    Pet::Tyrannosaur  => "RRRAAAAHHHH",
    Pet::Samoyed      => "*dog thoughts*",
    _  => "I'm cute, love me" // generic caption, works for any pet
    // _ wildcard pattern
  }

// There are many Shapes, but we only support "selecting"
// either some text, or everything in a rectangular area.
// You can't select an ellipse or trapezoid
match document.selection() {
  Shape::TextSpan(start, end) => paint_tet_selection(start, end),
  Shape::Rectangle(rect)      => paint_rect_selection(rect),
  _ => panic!("unexpected selection type")
}

fn check_move(current_hex: Hex, click: Point) -> game::Result<Hex> {
  match point_to_hex(click) {
    None  => 
          Err("That's not a game space"),
    Some(current_hex) => 
          Err("You are already there! You must click somewhere else."),
    Some(other_hex) =>
          Ok(other_hex)
/*
    Some(hex) => 
      if hex == current_hex {
        Err("You are already there!")
      } else {
        Ok(hex)
      }
*/
  }
}

// Tuple and struct patterns

fn describe_point(x: i32, y: i32) -> &'static str {
  use std::cmp::Ordering::*;
  match (x.cmp(&0), y.cmp(&0)) {
    (Equal, Equal) => "at the origin",
    (_,     Equal) => "on the x axis",
    (Equal, _    ) => "on the y axis",
    (Greater, Greater) => "in the first quadrant",
    (Less,    Greater) => "in the second quadrant",
    (Less,    Less   ) => "in the third quadrant",
    (Greater, Less   ) => "in the fourth quadrant",
    _                  => "Error: somewhere else"
  }
}

match balloon.location {
  Point { x:0, y:height } => println!("straight up {} meters", height),
  Point { x:x, y:y }      => println!("at ({}m, {}m)", x, y)
}

match get_account(id) {
  //...
  Some(Account {
    name, language, // <-- the 2 things we care about
    id: _, status: _, address: _, birthday: _, eye_color: _,
    pet: _, security_question: _, hashed_innermost_secret: _,
    is_adamantium_preferred_customer: _ 
  }) => language.show_custom_greeting(name),

  Some(Account { name, language, ..}) =>
    language.show_custom_greeting(name),
}

/* cannot work
match account {
  Account { name, language, .. } => {
    ui.greet(&name, & language);
    ui.show_settings(&account); // error: use of moved value `account`
  }
}
*/

match account {
  Account { ref name, ref language, .. } => {
    // ref borrow matched value instead of moving
    ui.greet(name, language);
    ui.show_settings(&account); // ok
  }
}

// ref mut to borrow mut reference
match line_result {
  Err(ref err) => log_err(err), // `err` is &Error (shared ref)
  Ok(ref mut line) => {         // `line` is &mut String (mut ref)
    trim_comments(line);        // modify the String in place
    handle(line);
  }
}

// reference pattern & pattern
match sphere.center() {
  &Point3d { x, y, z } => println!("x:{}, y:{}, z:{}", x, y, z),
  // xyz receive copies of the coordinates, leave original value intact

}

match friend.borrow_car() {
  Some(&Car { engine, .. }) => println!(""), // error: can't move out of borrow
  // use ref pattern to borrow a reference to part
  Some(&Car { ref engine, .. }) => println!(""), // ok, engine is a reference
  // ...
  None => {}
}

match chars.peek() {
  Some(&c) => println!("coming up: {:?}", c),
  None     => println!("end of chars"),
}

// Matching Multiple Possibilities
let at_end = 
  match chars.peek() {
    Some(&'\r') | Some(&'\n') | None => true,
    _ => false
  };

match next_char {
  '0'...'9'             => self.read_number(),
  'a'...'z' | 'A'...'Z' => self.read_word(),
  ' ' | '\t' | '\n'     => self.skip_whitespace(),
  _                     => self.handle_punctuation()
}

// Pattern Guards
match robot.last_known_location() {
  //Some(point) if self.distance_to(point) < 10 
  Some(ref point) if self.distance_to(point) < 10 
    => short_distance_strategy(point),
  Some(point)
    => long_distance_strategy(point),
  None
    => searching_strategy()
}

// @ patterns
match self.get_selection() {
  Shape::Rect(top_left, bottom_right) 
    => optimized_paint(&Shape::Rect(top_left, bottom_right)),
  rect @ Shape::Rect(..) => optimized_paint(&rect),
  other_shape
    => paint_outline(other_shape.get_outline()),
}

match chars.next() {
  Some(digit @ '0'...'9') = read_number(digit, chars),
  Some(word  @ 'a'...'z') | Some(word @ 'A'...'Z') => read_word(word, chars),
  _ => handle_punctuation(punct, chars)
}

// Where patterns are allowed

// unpack a struct into three new local variables
let Track { album, track_number, title, .. } = song;

// unpack a function argument that's a tuple
fn distance_to((x, y): (f64, f64)) -> f64 { }

// iterate over keys and values of a HashMap
for (id, document) in &cache_map {
  println!("Document #{}: {}", id, document.title);
}

// automatically dereference an argument to a closure
// (handy because sometimes other code passes you a reference 
// when you'd rather have a copy)
let sum = numbers.fold(0, |a, &num| a + num);

// handle just one enum variant specially
if let RoughTime::InTheFuture(_,_) = user.date_of_birth() {
  user.set_time_traveler(true);
}

// run some code only if a table loopup succeeds
if let Some(document) = cache_map.get(&id) {
  return send_cached_response(document);
}

// repeatedly try something until it succeeds
while let Err(err) = present_cheesy_anti_robot_task() {
  log_robot_attemp(err);
  // let the user try again (it might still be a human)
}

// manually loop over an iterator
while let Some(_) = lines.peek() {
  read_paragraph(&mut lines);
}

// Populating a binary tree
impl<T: Ord> BinaryTree<T> {
  fn add(&mut self, value: T) {
    match *self {
      BinaryTree::Empty =>
        *self = BinaryTree::NonEmpty(Box::new(TreeNode {
          element: value,
          left:  BinaryTree::Empty,
          right: BinaryTree::Empty,
        })),
      BinaryTree::NonEmpty(ref mut node) =>
        if value <= node.element {
          node.left.add(value);
        } else {
          node.right.add(value);
        }
    }
  }
}

// example code
let mut tree = BinaryTree::Empty;
tree.add("Mercury");
tree.add("Venus");
