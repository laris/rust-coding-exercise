//# crossbeam-channel = "*"
use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main() {
    let (tx, rx) = unbounded();
    let handle = thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{d}"),
                ThreadMsg::Sum(lhs, rhs) => println!("{lhs} + {rhs} = {}", lhs+rhs),
                ThreadMsg::Quit => {
                    println!("thread terminating");
                    break;
                },
            },
            Err(e) => {
                println!("disconnected: Err: {e:?}");
                break;
            }
        }
    });
    tx.send(ThreadMsg::PrintData("hello from main".into()));
    tx.send(ThreadMsg::Sum(10, 10));
    //tx.send(ThreadMsg::Quit);
    drop(tx);
    handle.join();
}
