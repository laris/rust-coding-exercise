struct Point<T> {
	x: T,
	y: T,
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let p = Point { x: 1.0, y: 1.0 };
	println!("Distance from origin: {}", p.distance_from_origin());
}