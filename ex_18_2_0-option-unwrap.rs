// The commoner has seen it all, and can handle any gift well.
// All gifts are handled explicitly using `match`.
fn give_commoner(gift: Option<&str>) {
  // Specify a course of action for each case.
  match gift {
      Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
      Some(inner)   => println!("{}? How nice.", inner),
      None          => println!("No gift? Oh well."),
  }
}

// Our sheltered royal will `panic` at the sight of snakes.
// All gifts are handled implicitly using `unwrap`.
fn give_royal(gift: Option<&str>) {
  // `unwrap` returns a `panic` when it receives a `None`.
  let inside = gift.unwrap();
  if inside == "snake" { panic!("AAAaaaaa!!!!"); }

  println!("I love {}s!!!!!", inside);
}

fn main() {
  let food  = Some("cabbage");
  let snake = Some("snake");
  let void  = None;

  give_commoner(food);
  give_commoner(snake);
  give_commoner(void);

  let bird = Some("robin");
  let nothing = None;

  give_royal(bird);
  give_royal(nothing);
}
