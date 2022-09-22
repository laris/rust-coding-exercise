enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64},
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad      => println!("page loaded"),
    WebEvent::PageUnload    => println!("page unloaded"),
    WebEvent::KeyPress(c)   => println!("pressed '{}'.", c),
    WebEvent::Paste(s)      => println!("pasted \"{}\".", s),
    WebEvent::Click{ x, y } => println!("clicked at x = {}, y = {}.", x, y),
  }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add      => x + y,
      Self::Subtract => x - y,
    }
  }
}
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted  = WebEvent::Paste("my text".to_owned());
  let click   = WebEvent::Click { x: 20, y: 80 };
  let load    = WebEvent::PageLoad;
  let unload  = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  let x_add = Operations::Add;
  let y_sub = Operations::Subtract;
  println!("x_add = {} + {} = {}", 1, 2, x_add.run(1, 2));
  println!("y_sub = {} - {} = {}", 2, 1, y_sub.run(2, 1));

}
