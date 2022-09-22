
fn customize_iterator_next() {
	struct Counter {
		count: u32,
	}
	impl Counter {
		fn new() -> Counter {
			Counter { count: 0 }
		}
	}
	impl Iterator for Counter {
		type Item = u32;
		fn next(&mut self) -> Option<Self::Item> {
			self.count += 1;
			if self.count < 6 {
				Some(self.count)
			} else {
				None
			}
		}
	}

	#[test]
	fn calling_next_directly() {
		let mut counter = Counter::new();
		assert_eq!(counter.next(), Some(1));
		assert_eq!(counter.next(), Some(2));
		assert_eq!(counter.next(), Some(3));
		assert_eq!(counter.next(), Some(4));
		assert_eq!(counter.next(), Some(5));
//		assert_eq!(counter.next(), Some(6));
		assert_eq!(counter.next(), None);
	}
}

fn main() {
	customize_iterator_next();
}