use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main fn Thread ID: {:02?}, {:?}", thread::current().id(), thread::current().name());
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::Builder::new()
        .name("child thread".into())
        .spawn(move || {
            println!("Child thread: start: {:02?}, {:?}", thread::current().id(), thread::current().name());
            println!("Child thread: start to do some work...");
            thread::sleep(Duration::from_secs(1));
            println!("Child thread: do some work done, cost 1 second");
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            println!("Child thread: change *started = true");
            *started = true;
            println!("Child thread: cvar.notify_one()");
            cvar.notify_one();
            println!("Child thread: *started = {}", *started);
            //*started = false;

    }).unwrap();

    println!("Main thread: wait for the thread to start up");
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    println!("Main thread: Before wait, *started = {}", *started);

    while !*started {
        println!("Main thread: while loop before cvar.wait(started): *started = {}", *started);
        started = cvar.wait(started).unwrap();
        println!("Main thread: while loop after cvar.wait(started): *started = {}", *started);
    }
    println!("Main thread: after wait, *started = {}", *started);

    // wait_while(&self, guard, cond: F)
    //let guard = cvar.wait_while(started, |pending| !*pending).unwrap();
    //println!("Main thread: after wait, *started = {}", *guard);
}
