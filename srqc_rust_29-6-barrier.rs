use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
	let barrier = Arc::new(Barrier::new(10));
	let mut handlers = vec![];
	for idx_thread in 0..10 {
		let c = barrier.clone();
		// THe same messages will be printed together.
		// You will NOT see any interleaving.
		let t = thread::spawn(move || {
			println!("Thread {} before wait", idx_thread);
			c.wait();
			println!("Thread {} after wait", idx_thread);
		});
		handlers.push(t);
	}

	for h in handlers {
		h.join().ok();
	}
}