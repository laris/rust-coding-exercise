use std::sync::Arc;
use std::thread;

fn main() {
	let numbers: Vec<_> = (0..100u32).collect();
	// Atomic ref counter, point to Vec
	let shared_numbers = Arc::new(numbers);

	// loop to create 10 threads
	for i in 0..10 {
		// copy arc, both Arc point to same one Vec
		let child_numbers = shared_numbers.clone();
		// move the pointer into new thread
		thread::spawn( move || {
			// in new thread, read the Vec via Arc
			let local_numbers = &child_numbers[..];
			println!("{} {}", i, local_numbers[99]);
		});
	}
}