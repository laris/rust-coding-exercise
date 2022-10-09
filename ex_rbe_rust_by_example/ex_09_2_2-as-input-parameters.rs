/* 
When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few traits. In order of decreasing restriction, they are:
  Fn: the closure captures by reference (&T)
  FnMut: the closure captures by mutable reference (&mut T)
  FnOnce: the closure captures by value (T)
*/

fn apply<F>(f: F) 
   where F: FnOnce() 
   {
     f();
   }

fn apply_to_3<F>(f: F) -> i32
   where F: Fn(i32) -> i32
   {
     f(3)
   }

fn main() {
  use std::mem;

  let greeting = "hello"; // &str
  let mut farewell = "goodbye".to_owned(); // String
  let diary = || {
    println!("I said {}.", greeting); // ref, Fn
    farewell.push_str("!!!");         // FnMut
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzzz");
    mem::drop(farewell); // FnOnce
  };

  apply(diary);

  let double = |x| 2 * x;
  println!("3 doubled: {}", apply_to_3(double));
}

/*
// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

*/