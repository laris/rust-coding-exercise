fn main() {
  let thor = ("Thor", true, 3500_u32);
  println!("{:?}", thor); // ("Thor", true, 3500)
  // type of thor: (&str, bool, u32)
  println!("{} - {} - {}", thor.0, thor.1, thor.2);

  // destructuring:
  let (name, _, power) = thor;
  println!("{} has {} power points", name, power);

  // one-element tuple:
  let one = (1,);

  // function returning a tuple and destructuring the return value:
  let (god, strengh) = increase_power(thor.0, thor.2);
  println!("This god {} has now strength {}.", god, strength);

  let pair_thor = (thor.0, thor.2);
  let (god, strength) = increase_power2(pair_thor);
  println!("This god {} have now trength {}.", god, strength);

  // swapping variables with a tuple:
  let mut n = 0;
  let mut m = 1;
  let (n, m) = (m, n);
  println!("n: {}, m: {}", n, m);
}

fn increase_power(name: &str, power: u32) -> (&str, u32) {
  if power > 1000 {
    (name, power *3)
  } else {
    (name, power *2)
  }
}

fn increase_power2((name, power): (&str, u32)) -> (&str, u32) {
  if power > 1000 {
    (name, power * 3)
  } else {
    (name, power * 2)
  }
}

// ("Thor", true, 3500)
// Thor - true - 3500
// Thor has 3500 power points
// This god Thor has now strength 10500 
// This god Thor has now strength 10500
// n: 1 m: 0