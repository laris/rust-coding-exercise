use std::f32::consts;

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";

fn main() {
  const MYPI: f32 = 3.14;
  println!("{}", MYPI);       // 3.14
  println!("{}", GAME_NAME);  // Monster Attack
  // use the PI value from the standard library:
  println!("{}", consts::PI); // 3.141593
}