use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
  println!("Main fn Thread ID: {:2?}, {:?}", thread::current().id(), thread::current().name());
  let barrier = Arc::new(Barrier::new(10));
  let mut handlers = vec![];
  for idx_thread in 0..10 {
    let c = barrier.clone();
    // THe same messages will be printed together.
    // You will NOT see any interleaving.
    let t = thread::spawn(move || {
      println!("Before: Thread ID: {:2?} with idx_thread: {idx_thread}, Thread name: {:?}", thread::current().id(), thread::current().name());
      c.wait();
      println!("After:  Thread ID: {:2?} with idx_thread: {idx_thread}, Thread name: {:?}", thread::current().id(), thread::current().name());
    });
    handlers.push(t);
  }

  for h in handlers {
    h.join().ok();
  }
}
