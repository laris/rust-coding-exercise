use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

struct Table {
	forks: Vec<Mutex<()>>,
}

impl Philosopher {
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(),
			left: left,
			right: right,
		}
	}
	fn eat(&self, table: &Table) {
		let _left = table.forks[self.left].lock().unwrap();
		println!("{}\t take left fork.", self.name);
		thread::sleep(Duration::from_secs(2));
		let _right = table.forks[self.right].lock().unwrap();
		println!("{}\t take right fork.", self.name);
		thread::sleep(Duration::from_secs(1));
		println!("{}\t is done eating.", self.name);

	}
}

fn main() {
	let table = Arc::new(Table { forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
	]});

	let philosophers = vec![
		Philosopher::new("Judith Butler",   0, 1),
		Philosopher::new("Gilles Deleuze",  1, 2),
		Philosopher::new("Karl Marx",       2, 3),
		Philosopher::new("Emma Goldman",    3, 4),
		Philosopher::new("Michel Foucault", 4, 0),
	];

	let handles: Vec<_> = philosophers.into_iter().map( |p| {
		let table = table.clone();
		thread::spawn(move || {
			p.eat(&table);
		})
	}).collect();

	for h in handles {
		h.join().unwrap();
	}
}
