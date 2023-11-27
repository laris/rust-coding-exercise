use std::thread;
use std::time::Duration;

fn main() {
    /*
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
     */

     /*
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();


    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    */

    /*

    // move
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
      */

    /*
    let new_thread = thread::spawn(move || {
        thread::spawn(move || {
            let mut i: u64 = 0;
            loop {
                println!("I am a new thread. {i}");
                i += 1;
            }
        })
    });

    new_thread.join().unwrap();
    println!("child thread finished!");

    thread::sleep(Duration::from_secs(30));
     */

     // 多线程的开销, CAS 无锁实现
     /*
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut ht = HashMap::new();
    let num_threads = 100;
    let adds_per_thread = 10;
    for i in 0..num_threads {
        let ht = Arc::clone(&ht);

        let handle = thread::spawn(move || {
            for j in 0..adds_per_thread {
                let key = thread_rng.gen::<u32>();
                let value = thread_rng.gen::<u32>();
                ht.set_item(key, value);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
     */

     // thread barrier 线程屏障(Barrier)
     /*
      */
    use std::sync::{Arc, Barrier};
    use std::thread;

    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

}