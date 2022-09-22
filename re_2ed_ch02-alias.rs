type MagicPower = u16;

fn main() {
  let mut run: MagicPower = 7800;

  run = 65535;
//  run = 78000;
  // error: literal out of range for `u16`
  // note: the literal `78000` does not fit into the type `u16` whose range is `0..=65535`

}