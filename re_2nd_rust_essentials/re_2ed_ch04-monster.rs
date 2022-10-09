struct Monster {
  health: i32,
  damage: i32
}

fn main() {
  let m = Monster { health: 10, damage: 20 };

  println!("{}", m.health); // 10
  println!("{}", m.damage); // 20
}

