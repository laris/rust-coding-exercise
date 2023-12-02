use std::thread;
fn main() {
    println!("main thread, Thread::current(): {:?}", thread::current());
    let handle = thread::spawn(move || {
        println!("thread1, Thread::current(): {:?}", thread::current());
    });

    let builder2 = thread::Builder::new().name("thread2".into());
    let handle2 = builder2.spawn(move || {
        println!("thread2, \nThread::current(): {:?},\n 
                    thread::current().name(): {:?},\n 
                    thread::current().id(): {:?}\n", 
                    thread::current(), 
                    thread::current().id(), 
                    thread::current().name());
    }).unwrap();

    let handle3 = thread::Builder::new()
                    .name("thread3".into())
                    .spawn(move || {
        println!("thread3, \nThread::current(): {:?},\n 
                    thread::current().name(): {:?},\n 
                    thread::current().id(): {:?}\n", 
                    thread::current(), 
                    thread::current().id(), 
                    thread::current().name());
    }).unwrap();

    handle.join();
    handle2.join();
    handle3.join();
}
