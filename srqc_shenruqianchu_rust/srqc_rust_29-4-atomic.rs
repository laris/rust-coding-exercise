use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;

const COUNT: u32 = 1_000_000;

fn main() {
	// Atomic type provide thread safety and internal mutable
	let global = Arc::new(AtomicIsize::new(0));

	let clone1 = global.clone();
	let thread1 = thread::spawn(move || {
		for _ in 0..COUNT {
			clone1.fetch_add(1, Ordering::SeqCst);
		}
	});

	let clone2 = global.clone();
	let thread2 = thread::spawn(move || {
		for _ in 0..COUNT {
			clone2.fetch_sub(1, Ordering::SeqCst);
		}
	});
/*
	let global2 = Arc::new(AtomicIsize::new(0));

	let clone3 = global2.clone();
	let thread3 = thread::spawn(move || {
		for _ in 0..COUNT {
			let mut value = clone3.load(Ordering::SeqCst);
			value += 1;
			clone3.store(value, Ordering::SeqCst);
		}
	});

	let clone4 = global2.clone();
	let thread4 = thread::spawn(move || {
		for _ in 0..COUNT {
			let mut value = clone4.load(Ordering::SeqCst);
			value -= 1;
			clone3.store(value, Ordering::SeqCst);
		}
	});
*/
	thread1.join().ok();
	thread2.join().ok();
	println!("final value: {:?}", global);
//	thread3.join().ok();
//	thread4.join().ok();
//	println!("final value: {:?}", global2);
}