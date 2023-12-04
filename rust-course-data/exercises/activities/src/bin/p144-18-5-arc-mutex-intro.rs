//# parking_lot = "*"

use parking_lot::Mutex;
use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::{SystemTime, Duration};

struct Counter(usize);

fn main() {
    let counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread_1_counter = Arc::clone(&shared_counter);
    let thread_2_counter = shared_counter.clone();

    println!("main before thread: {:?}", SystemTime::now());
    let thread_1 = thread::spawn(move || {
        println!("thread1 start: {:?}", SystemTime::now());
        let mut counter = thread_1_counter.lock();
        println!("thread1 locked: {:?}", SystemTime::now());
        thread::sleep(Duration::from_secs(3));
        println!("thread1 after sleep: {:?}", SystemTime::now());
        counter.0 += 1;
        println!("thread1 quit: {:?}", SystemTime::now());
    });
    let thread_2 = thread::spawn(move || {
        println!("thread2 start: {:?}", SystemTime::now());
        let mut counter = thread_2_counter.lock();
        println!("thread2 after lock: {:?}", SystemTime::now());
        thread::sleep(Duration::from_secs(3));
        println!("thread2 after sleep: {:?}", SystemTime::now());
        counter.0 += 1;
        println!("thread2 quit: {:?}", SystemTime::now());
    });
    
    println!("main after thread: {:?}", SystemTime::now());

    thread_1.join().and_then(|_| thread_2.join());
    println!("main after join : {:?}", SystemTime::now());
    println!("{}", shared_counter.lock().0);
}
