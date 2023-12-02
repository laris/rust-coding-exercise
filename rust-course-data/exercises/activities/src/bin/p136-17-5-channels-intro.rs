//# crossbeam-channel = "*"
use crossbeam_channel::unbounded;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = unbounded();
    tx.send("hello, channel!");
    match rx.recv() {
        Ok(msg) => println!("{msg}"),
        Err(e)  => println!("{e:?}"),
    }
    //demo2
    let (s, r) = unbounded();
    let r2 = r.clone();
    let handle = thread::spawn(move || 
                match r.recv() {
                    Ok(msg) => println!("Thread: {msg}"),
                    Err(e)  => println!("{e:?}"),
    });
    let handle2 = thread::spawn(move || 
                match r2.recv() {
                    Ok(msg) => println!("Thread2: {msg}"),
                    Err(e)  => println!("{e:?}"),
    });
    s.send("Hallo from main!")?;
    s.send("Hallo2 from main!")?;
    handle.join();
    handle2.join();
    Ok(())
}
