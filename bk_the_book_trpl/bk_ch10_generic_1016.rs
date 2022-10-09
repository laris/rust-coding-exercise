use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self {
			x,
			y,
		}
	}
}

impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

fn main() {
	let p0 = Pair { x: 0u8, y: 0u8, };
	let p1 = Pair::new(0i32, 0i32);
	let p2 = Pair::new(100u64, 200u64);
	p2.cmp_display();

	//let p3 = Pair::new(vec![1], vec![2]);
	//note: the following trait bounds were not satisfied:
	//`Vec<{integer}>: std::fmt::Display`

}