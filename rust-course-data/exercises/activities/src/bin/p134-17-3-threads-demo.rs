use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let iterations = 10; // copyable, so move = copy
    let thread_a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A:{i}");
        }
    });
    let thread_b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("--------B:{i}");
        }
    });
    thread_a.join();
    thread_b.join();
    // demo2
    let return_value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
    println!("Waiting on thread");
    match return_value.join() {
        Ok(n) => println!("value: {n}"),
        Err(e) => println!("error joining thread: {e:?}"),
    }

    // demo 3
    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });
    match caps.join() {
        Ok(n) => println!("value: {n:?}"),
        Err(e) => println!("error joining thread: {e:?}"),
    }


}
