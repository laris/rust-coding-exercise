
struct Guess {
	value: i32,
}

impl Guess {
	fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}
		Guess { value }
	}

	fn value(&self) -> i32 {
		self.value
	}
}

fn main() {
	let guess1 = Guess::new(10);
	println!("guess1: {}", guess1.value());
	let guess2 = Guess::new(0);

}