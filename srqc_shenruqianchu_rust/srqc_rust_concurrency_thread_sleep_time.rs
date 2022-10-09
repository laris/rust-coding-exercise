#[allow(unused_variables)]

fn main() {

    // sync loop, report iteration timestamp and sleep 1 millis
    /*
    use std::time::{Duration, SystemTime};
    use std::thread;
    for i in 0..1000 {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        thread::sleep(Duration::from_millis(1)); // sleep 1 millis in this one thread
        println!("{:>6}, {:?}", i, now.unwrap().as_millis());
    }
     */

    // thread demo
    /*
    use std::thread;
    use std::time::Duration;

    let t = thread::Builder::new()
        .name("Child thread 1".to_string())
        .spawn(move || {
            println!("Enter child thread.");
            thread::park();
            println!("Resume child thread.");
        }
        ).unwrap();

    println!("Spawn a thread");
    thread::sleep(Duration::new(5, 0));
    t.thread().unpark();
    t.join();
    println!("child thread finished.");
     */

    // std::thread doc
    /*
    use std::thread;
    let thread_join_handle = thread::spawn(move || {
        println!("current thread: {:?}", thread::current());
        // some work here
    });

    // some work here
    let res = thread_join_handle.join();
    println!("Join result: {:?}", res);
    println!("thread 1 meta: {:?}", res);

    // The join method returns a thread::Result containing Ok of the final value produced by the spawned thread, or Err of the value given to a call to panic! if the thread panicked.

    let th2_handle = thread::Builder::new().name("thread1".to_string()).spawn(move || {
        println!("thread1: Hello workd!");
        println!("current thread: {:?}", thread::current());
    });
    // let res2 = th2_handle.join();
    // println!("Join result: {:?}", res2);
    println!("thread 2 meta: {:?}", th2_handle);

    println!("current thread: {:?}", thread::current());
    */


    // spwan doc
    /*
    use std::thread;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        tx.send("Hello, thread".to_owned())
            .expect("Unable to send to channel");
    });

    let receiver = thread::spawn(move || {
        let value = rx.recv().expect("Unable to receive from channel");
        println!("{value}");
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");

    let computation = thread::spawn(|| {
        // some expensive computation
        42
    });

    let result = computation.join().unwrap();
    println!("Result: {:?}", result);

    // JoinHandle
    let join_handle: thread::JoinHandle<_> = thread::spawn(|| {
        // some work here
        println!("join_handle thread");
    });
    println!("{join_handle:?}");

    let builder = thread::Builder::new().name("join_handle2".into());
    let join_handle2: thread::JoinHandle<_> = builder.spawn(|| {
        println!("join_handle2 thread");
    }).unwrap();


    use std::time::Duration;
    let original_thread = thread::spawn(|| {
        let _detached_thread = thread::spawn(|| {
            // Here we sleep to make sure that the first thread returns before.
            thread::sleep(Duration::from_millis(10));
            // This will be called, even though the JoinHandle is dropped.
            println!("♫ Still alive ♫");
        });
    });
    original_thread.join().expect("The thread being joined has panicked");
    println!("Original thread is joined.");
    // We make sure that the new thread has time to run, before the main
    // thread returns.
    thread::sleep(Duration::from_millis(1000));

    let builder3 = thread::Builder::new();
    let join_handle3: thread::JoinHandle<_> = builder3.spawn(|| {
        println!("join_handle3 thread");
    }).unwrap();
    let thread = join_handle3.thread();
    println!("thread id: {:?}", thread.id());

    // Thread struct
    let other_thread = thread::spawn(|| {
        thread::current().id()
    });
    println!("other_thread join_handle: {:?}", other_thread);
    let other_thread_id = other_thread.join().unwrap();
    println!("other_thread_id: {:?}", other_thread_id);
    //println!("other_thread_id: {:?}", other_thread_id.as_u64()); // nightly

    // Thread::name()
    let handle4 = thread::Builder::new().spawn(|| {
        println!("Thread default have no name specified: {:?}", thread::current().name());
    }).unwrap();
    handle4.join().unwrap();

    let handle5 = thread::Builder::new()
                        .name("foooooo".into())
                        .spawn(|| {
                            println!("Thread handle5's name: {:?}", thread::current().name());
                        }).unwrap();
    handle5.join().unwrap();
     */

    // unpark
    /*
    use std::thread;
    use std::time::Duration;
    let parked_thread = thread::Builder::new()
        .name("parked_thread".into())
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        }).unwrap();

    // let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));
    println!("Unpark the thread");
    //parked_thread.thread().unpark();
    thread::Thread::unpark(&parked_thread.thread());
    parked_thread.join().unwrap();
     */

    // park
    /*
    */
    use std::thread;
    use std::sync::{Arc, atomic::{Ordering, AtomicBool}};
    use std::time::Duration;

    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = Arc::clone(&flag);

    let parked_thread = thread::spawn(move || {
        // We want to wait until the flag is set. We *could* just spin, but using
        // park/unpark is more efficient.
        while !flag2.load(Ordering::Acquire) {
            println!("Parking thread");
            thread::park();
            // We *could* get here spuriously, i.e., way before the 10ms below are over!
            // But that is no problem, we are in a loop until the flag is set anyway.
            println!("Thread unparked");
        }
        println!("Flag received");
    });

    // let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    // Set the flag, and let the thread wake up.
    // There is no race condition here, if `unpark`
    // happens first, `park` will return immediately.
    // Hence there is no risk of a deadlock.
    flag.store(true, Ordering::Release);
    println!("Unpark the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();

}