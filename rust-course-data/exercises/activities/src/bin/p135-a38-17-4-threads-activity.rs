// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    use std::thread;
    let thread_hello = thread::spawn(move || {
        print!("{}", msg_hello());
    });
    let thread_thread = thread::spawn(move || {
        print!("{}", msg_thread());
    });
    let thread_excited = thread::spawn(move || {
        print!("{}", msg_excited());
    });
    // demo
    thread_hello.join();
    thread_thread.join();
    thread_excited.join();
    println!("");
    let msg_hello = thread::spawn(move || msg_hello());
    let msg_thread = thread::spawn(move || msg_thread());
    let msg_excited = thread::spawn(move || msg_excited());
    let msg_hello =  msg_hello.join().expect("failed to join msg hello");
    let msg_thread = msg_thread.join().expect("failed to join msg thread");
    let msg_excited= msg_excited.join().expect("failed to join msg excited");
    println!("{msg_hello}{msg_thread}{msg_excited}");
}
