assert_eq!(gcd(6, 10), 2);

match (&gcd(6, 10), &2) {
  (left_val, right_val) => {
    if !(*left_val == *right_val) {
      panic!("assertion failed: `(left == right)`, \
            (left: `{:?}`, right: `{:?}`)", left_val, right_val);
    }
  }
}

macro_rules! assert_eq {
  ($left:expr, $right:expr) => ({
    match (&$left, &$right) {
      (left_val, right_val) => {
        if !(*left_val == *right_val) {
          panic!("assertion failed: `(left == right)` \
                (left: `{:?}`, right: `{:?}`)",
                left_val, right_val)
        }
      }
    }
  });
}

// Repetition
// Repeat a value N times
let buffer = vec![0_u8; 1000];
// A list of values, separated by commas
let numbers = vec!["udon", "ramen", "soba"];

macro_rules! vec {
  ($elem:expr ; $n:expr) => {
    ::std::vec::from_elem($elem, $n)
  };
  ($($x:expr), *) => { 
  // syntax $( PATTERN ),* is used to match any comma-separated list
    <[_]>::into_vec(Box::new([$($x), *]))
  };
  ($($x:expr), + ,) => { // use + to require at least one match
    vec![$($x), *]
  };
}
/*
Pattern     Meaning
$( ... )*   Match 0 or more times with no separator
$( ... ),*  Match 0 or more times, separated by commas
$( ... );*  Match 0 or more times, separated by semicolons
$( ... )+   Match 1 or more times with no separator
$( ... ),+  Match 1 or more times, separated by commas
$( ... );+  Match 1 or more times, separated by semicolons
*/
// code fragment $x is not just a single expression but a list of expressions
<[_]>::into_vec(Box::new([ $( $x ),* ]))
// This code creates a boxed array, and then 
// uses the [T]::into_vec method to convert the boxed array to a vector.
// <[_]>, is an unusual way to write the type “slice of something”
( $( $x: expr ),* ) => {
  {
    let mut v = Vec::new();
    $( v.push($x); )*
    v
  }
};

( $($x: expr), + ,) => {  // if trailing comma is present,
  vec![$($x),*]           // retry without it
};

// Built-In Macros
/*
file!() expands to a string literal: the current filename

line!() column!() expand to u32 literals giving the current
line couting from 1 and column counting from 0

stringify!(..tokens...) expands to a string literal containing the given tokens.
stringify!(line!()) expands to the string "line!()"

concat!(str0, str1, ...) expands to a single string literal made bye concatenating its arguments.

cfg!(...) expands to a Boolean constant
cfg!(debug_assertions)

env!("VAR_NAME") expands to a string
let version = env!("CARGO_PKG_VERSION");

option_env!("VAR_NAME")  is the same as env! except that it returns an Option<&'static str> that is None if the specified variable is not set.

include!("file.rs") expands to the contents of the specified file
include_str!("file.txt") expands to a &'static str containing the text of the specified file
const COMPOSITOR_SHADER: &str = 
      include_str!("../resources/compositor.glsl");
include_bytes!("file.dat") is the same except the file is treated as binary data, not UTF-8 text. The result is a &'static [u8].

In all cases, if the filename is a relative path, 
it’s resolved relative to the directory that contains the current file.
*/


