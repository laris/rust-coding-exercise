//#tokio = { version = "1", features = ["rt", "macros"] }
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Waker, Context};
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Eq, PartialEq)]
enum InnerState {
    Init,
    Sleeping,
    Done,
}
struct State {
    waker: Option<Waker>,
    inner_state: InnerState,
}

struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}

impl SleepFuture {
    fn new(duration: Duration) -> Self {
        Self { 
            duration,
            state: Arc::new(Mutex::new(
                State {
                waker: None,
                inner_state: InnerState::Init,
            })),
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard_state = self.state.lock().unwrap();

        println!("jump into poll fn...");
        if guard_state.inner_state == InnerState::Done {
            println!("InnerState::Done Poll::Ready");
            return Poll::Ready(());
        }

        if guard_state.inner_state == InnerState::Init {
            println!("InnerState::Init");
            guard_state.waker = Some(cx.waker().clone());
            guard_state.inner_state = InnerState::Sleeping;
            println!("InnerState::Sleeping");
            let duration = self.duration;
            let state_cloned = Arc::clone(&self.state);
            thread::spawn(move || {
                println!("before sleep duration: {duration:?}");
                thread::sleep(duration);
                println!("after sleep duration: {duration:?}");
                let mut guard_state = state_cloned.lock().unwrap();
                guard_state.inner_state = InnerState::Done;
                println!("before wake, InnerState::Done");
                if let Some(waker) = guard_state.waker.take() {
                    waker.wake_by_ref();
                }
                println!("after wake, InnerState::Done");
            });
        }
        guard_state.waker = Some(cx.waker().clone());
        //if self.state.waker != None && !cx.waker().will_wake(self.state.waker.unwrap()) {
        //    self.state.waker = Some(cx.waker().clone());
        //}
        println!("Poll::Pending");
        Poll::Pending
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("start main.....");
    SleepFuture::new(Duration::from_secs(5)).await;
    println!("end main.....");
}
