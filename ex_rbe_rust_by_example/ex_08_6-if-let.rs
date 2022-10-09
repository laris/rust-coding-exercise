/*
// Make `optional` of type `Option<i32>`
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ Needed 2 indentations just so we could destructure
        // `i` from the option.
    },
    _ => {},
    // ^ Required because `match` is exhaustive. Doesn't it seem
    // like wasted space?
};
*/

fn main() {
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  if let Some(i) = letter {
    println!("Matched {:?}", i);
  } else {
    println!("Didn't match a number. Let's go with a letter!");
  }

  let i_like_letters = false;
  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter!");
  } else {
    println!("I don't like letters. Let's go with an emoticon :)!");
  }

  enum Foo {
    Bar,
    Baz,
    Qux(u32),
  }

  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  // if Foo::Bar == a { // fail to complie, instance of enum cannot be equated
  if let Foo::Bar = a {
    println!("a is foobar");
  }
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
      println!("b is foobar");
  }

  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  if let Foo::Qux(value @ 100) = c {
    println!("c is one hundred");
  }


}
