// [Rust Async 异步编程 简易教程](https://www.bilibili.com/video/BV16r4y187P4)
/*
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello before reading file !");
    let file1_contents = read_from_file1();
    println!("{:?}", file1_contents);
    println!("Hello after reading file1 !");

    let file2_contents = read_from_file2();
    println!("{:?}", file2_contents);
    println!("Hello after reading file1 !");
}

fn read_from_file1() -> String {
    sleep(Duration::new(4,0));
    String::from("Hello, there from file 1")
}

fn read_from_file2() -> String {
    sleep(Duration::new(2,0));
    String::from("Hello, there from file 2")
}
*/
/*
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello before reading file!");
    let handle1 = thread::spawn(|| {
        let file1_contents = read_from_file1();
        println!("{:?}", file1_contents);
    });
    let handle2 = thread::spawn(|| {
        let file2_contents = read_from_file2();
        println!("{:?}", file2_contents);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
fn read_from_file1() -> String {
    sleep(Duration::new(4,0));
    String::from("Hello, there from file 1")
}

fn read_from_file2() -> String {
    sleep(Duration::new(2,0));
    String::from("Hello, there from file 2")
}
*/

// tokio
/*
use std::thread::sleep;
use std::time::Duration;
#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn(async {
        let _file1_contents = read_from_file1().await;
    });

    let h2 = tokio::spawn(async {
        let _file2_contents = read_from_file2().await;
    });

    let _ = tokio::join!(h1, h2);
}

/*
async fn read_from_file1() -> String {
    sleep(Duration::new(4,0));
    println!("{:?}", "Processing file 1");
    String::from("Hello, there from file 1")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(2,0));
    println!("{:?}", "Processing file 2");
    String::from("Hello, there from file 2")
}
*/

// Future
// use std::thread::sleep;
// use std::time::Duration;
use std::future::Future;

fn read_from_file1() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(4,0));
        println!("{:?}", "Processing file 1");
        String::from("Hello, there from file 1")
    }
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(3,0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}

*/

/*
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Tokio! Stop polling me");
        cx.waker().wake_by_ref(); // first change
        // Poll::Pending // orig design
        Poll::Ready(String::from("Hello, there from file 1")) // 2nd change
        // why the Poll::Ready cannot print in stdout?
    }
}

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn(async {
        let future1 = ReadFileFuture {};
        future1.await
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2,0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};

struct AsyncTimer { expiration_time: Instant, }

impl Future for AsyncTimer {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("Hello, it's not yet time for Future 1, Going to sleep");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            std::thread::spawn(move || {
                println!("start new thread with sleep blocking");
                let curr_time = Instant::now();
                if curr_time < expiration_time {
                    std::thread::sleep(expiration_time - curr_time);
                }
                println!("end sleep and wake.wake()");
                waker.wake();
                println!("end wake.wake()");
            });
            println!("end new thread and return Poll::Pending");
            Poll::Pending
        }
    }
}
#[tokio::main]
async fn main() {
    println!("start tokio::main");
    let h1 = tokio::spawn(async {
        println!("start future1 handle");
        // let future1 = AsyncTimer{ expiration_time: Instant::now() + Duration::from_millis(4000), };
        let future1 = AsyncTimer{ expiration_time: Instant::now() + Duration::from_secs(4), };
        println!("generate future1 handle, then run future1.await");
        println!("{:?}", future1.await);
        println!("after future1.await");
    });

    let h2 = tokio::spawn(async {
        println!("start future2 handle");
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
        println!("end future2 handle");
    });

    let _ = tokio::join!(h1, h2);
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(4,0));
        String::from("Future 2 has completed")
    }
}