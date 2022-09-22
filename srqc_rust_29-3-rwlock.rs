use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

const COUNT: u32 = 1_000_000;

fn main() {
	let global = Arc::new(RwLock::new(0));

	let clone1 = global.clone();
	let thread1 = thread::spawn(move || {
		for _ in 0..COUNT {
			let mut value = clone1.write().unwrap();
			*value += 1;
		}
	});

	let clone2 = global.clone();
	let thread2 = thread::spawn(move || {
		for _ in 0..COUNT {
			let mut value = clone2.write().unwrap();
			*value -= 1;
		}
	});

	thread1.join().ok();
	thread2.join().ok();
	println!("final value: {:?}", global);
}