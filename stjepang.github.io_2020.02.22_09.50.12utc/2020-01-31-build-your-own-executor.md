---
layout: post
title:  "Build your own executor"
---

Now that we’ve [built the](https://stjepang.github.io/2020/01/25/build-your-own-block-on.html) [`block_on()`](https://stjepang.github.io/2020/01/25/build-your-own-block-on.html) [function](https://stjepang.github.io/2020/01/25/build-your-own-block-on.html), it’s time to take one step further and turn it into a real executor. We want our executor to run not just one future at a time but many futures concurrently!

This blog post is inspired by [`juliex`](https://github.com/withoutboats/juliex), a minimal executor and one of the first that pioneered async/await support in Rust. Today we’re writing a more modern and cleaner version of `juliex` from scratch.

The goal for our executor is to have only simple and completely safe code while delivering performance that rivals existing best-in-class executors.

Crates we’ll use as dependencies are [`crossbeam`](https://docs.rs/crossbeam), [`async-task`](https://docs.rs/async-task), [`once_cell`](https://docs.rs/once_cell), [`futures`](https://docs.rs/futures), and [`num_cpus`](https://docs.rs/num_cpus).


## The interface

The executor is going to have just one function that spawns a future:

```rust
fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    todo!()
}
```  

The returned `JoinHandle<R>` is a type that implements `Future<Output = R>` and retrieves its output once the task has completed.

Note the similarities between this `spawn()` function and [`std::thread::spawn()`](https://doc.rust-lang.org/nightly/std/thread/fn.spawn.html) — they’re almost equivalent, except one spawns an async task and the other spawns a thread.

Here’s a simple example spawning a task and awaiting its output:

```rust
fn main() {
    futures::executor::block_on(async {
        let handle = spawn(async { 1 + 2 });
        assert_eq!(handle.await, 3);
    });
}
```

## Passing the output to JoinHandle

Since `JoinHandle` is a type implementing `Future`, let’s be lazy for now and simply define it as an alias for a pinned boxed future:

```rust
type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;
```

This works for now but don’t fret, later on we’ll rewrite it cleanly as a fresh `struct` and implement `Future` for it manually.

The output of the spawned future has to be sent to `JoinHandle` somehow. One way to do that is to create a [oneshot channel](https://docs.rs/futures/0.3.1/futures/channel/oneshot/index.html) and send the output through the channel when the future completes. The `JoinHandle` is then a future that awaits a message from the channel:

```rust
use futures::channel::oneshot;

fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (s, r) = oneshot::channel();
    let future = async move {
        let _ = s.send(future.await);
    };

    todo!()

    Box::pin(async { r.await.unwrap() })
}
```

The next step is allocating the wrapper future on the heap and pushing it into some kind of global task queue so that it gets processed by the executor. We call such an allocated future a *task*.


## The anatomy of a task

A task consists of a future and its state. We need to keep track of the state to know whether the task is scheduled for running, whether it is currently running, whether it has completed, and so on.

Here’s the definition for our `Task` type:

```rust
struct Task {
    state: AtomicUsize,
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
}
```

We haven’t decided yet what exactly `state` is, but it will be some kind of [`AtomicUsize`](https://doc.rust-lang.org/nightly/std/sync/atomic/struct.AtomicUsize.html) that can be updated from any thread. Let’s figure that out later.

The output type of the future is `()` — that is because the `spawn()` function has wrapped the original future into one that sends the output into the oneshot channel and then simply returns with `()`.

The future is pinned on the heap. It has to be because only pinned futures can be polled. But why is it also wrapped into a `Mutex`?

Every `Waker` associated with the task will hold a `Task` reference so that it can wake the task by pushing it into the global task queue. Therein lies the problem: `Task` instances are shared among threads, but polling the future requires mutable access to it. Solution: we wrap the future into a mutex to get mutable access to it.

If all this sounds confusing, don’t worry, it’ll make more sense once we finish the whole executor!

Moving on. Let’s complete the `spawn()` function by allocating a `Task` holding the future and its state:

```rust
fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (s, r) = oneshot::channel();
    let future = async move {
        let _ = s.send(future.await);
    };

    let task = Arc::new(Task {
        state: AtomicUsize::new(0),
        future: Mutex::new(Box::pin(future)),
    });
    QUEUE.send(task).unwrap();

    Box::pin(async { r.await.unwrap() })
}
```

Once the task is allocated, we push it into `QUEUE`, the global queue containing runnable tasks. The `spawn()` function is now complete, so let’s define `QUEUE` next…


## Executor threads

Since we’re building an executor, there must be a background thread pool that takes runnable tasks from the queue and runs them, i.e. polls their futures.

Let’s define the global task queue and spawn executor threads the first time it is initialized:

```rust
use crossbeam::channel;
use once_cell::sync::Lazy;

static QUEUE: Lazy<channel::Sender<Arc<Task>>> = Lazy::new(|| {
    let (sender, receiver) = channel::unbounded::<Arc<Task>>();

    for _ in 0..num_cpus::get().max(1) {
        let receiver = receiver.clone();
        thread::spawn(move || receiver.iter().for_each(|task| task.run()));
    }

    sender
});
```

Pretty simple — an executor thread is literally a one-liner! So the task queue is an unbounded channel, while executor threads receive tasks from this channel and run each one of them.

The number of executor threads equals the number of cores on the system, which is retrieved by the `num_cpus` crate.

Now that we have the task queue and a thread pool, the last missing piece to implement is the `run()` method.


## Task execution

Running a task simply means polling its future. We already know how to poll futures from the [previous blog post](https://stjepang.github.io/2020/01/25/build-your-own-block-on.html) where we implemented `block_on()`, which is going to help.

The `run()`  method looks something like this:

```rust
impl Task {
    fn run(self: Arc<Task>) {
        let waker = todo!();

        let cx = &mut Context::from_waker(&waker);
        self.future.try_lock().unwrap().as_mut().poll(cx);
    }
}
```

Note that we need to lock the future to get mutable access and poll it. By design, no other thread will hold the lock at the same time, so `try_lock()` must always succeed.

But how do we create a waker? We’re going to use [`async_task::waker_fn()`](https://docs.rs/async-task/1.3.0/async_task/fn.waker_fn.html) like the last time, but what is the wake function supposed to do?

We can’t push an `Arc<Task>` into `QUEUE` just like that. Here are potential race conditions we should think about:


- What if a task completes and is then woken? A `Waker` can outlive its associated future, and we don’t want to end up with completed tasks in the queue.
- What if a task is woken twice in a row before it gets run? We don’t want to have two references to the same task in the queue.
- What if a task is woken while it’s running? If we push a reference to it into the queue, another executor thread might take it and attempt to run it at the same time.

If we think hard about it, we come up with two simple rules that solve all of these problems elegantly:


1. The wake function schedules the task if it wasn’t woken already and if it’s not currently running.
2. If the task was woken while it was running, the executor thread reschedules it.

Let's sketch these rules out:

```rust
impl Task {
    fn run(self: Arc<Task>) {
        let waker = async_task::waker_fn(|| {
            todo!("schedule if the task is not woken already and is not running");
        });

        let cx = &mut Context::from_waker(&waker);
        self.future.try_lock().unwrap().as_mut().poll(cx);

        todo!("schedule if the task was woken while running");
    }
}
```

Remember the `state` field of type  `AtomicUsize` we defined inside `Task`? Now is the time to store some useful data in it. There are two pieces of information we care about tasks that will help us implement waking:

1. Has the task been woken already?
2. Is the task currently running?

Both of those are true/false values, and we can represent them with two bits inside the `state` field:

```rust
const WOKEN: usize = 0b01;
const RUNNING: usize = 0b10;
```

The wake function sets the `WOKEN` bit. If both bits have previously been 0 (i.e. the task was neither woken nor running), then we schedule the task by pushing its reference into the queue:

```rust
let task = self.clone();
let waker = async_task::waker_fn(move || {
    if task.state.fetch_or(WOKEN, Ordering::SeqCst) == 0 {
        QUEUE.send(task.clone()).unwrap();
    }
});
```

Just before polling the future, we unset the `WOKEN` bit and set the `RUNNING` bit:

```rust
self.state.store(RUNNING, Ordering::SeqCst);
let cx = &mut Context::from_waker(&waker);
let poll = self.future.try_lock().unwrap().as_mut().poll(cx);
```

After polling the future, we unset the `RUNNING` bit and check if the previous state had bits `WOKEN` and `RUNNING` set (i.e. the task was woken while running). If so, we reschedule the task:

```rust
if poll.is_pending() {
    if self.state.fetch_and(!RUNNING, Ordering::SeqCst) == WOKEN | RUNNING {
        QUEUE.send(self).unwrap();
    }
}
```

Interestingly, if the task completes (i.e. its future is not pending anymore), we leave it in the running state forever. That way future wakeups can’t reschedule the completed task.

And that’s all. Done! We have a real executor now — see the complete implementation in [`v1.rs`](https://github.com/stjepang/byo-executor/blob/master/examples/v1.rs).


## A touch of magic

If you found the `Task` struct and its state transitions intimidating, I feel you. But there is good news. You’ll be relieved to hear none of that mess needs to be done by hand because `async-task` can do it for us!

We basically need to replace `Arc<Task>` with [`async_task::Task<()>`](https://docs.rs/async-task/1.3.0/async_task/struct.Task.html) and replace the oneshot channel with [`async_task::JoinHandle<()>`](https://docs.rs/async-task/1.3.0/async_task/struct.JoinHandle.html).

This is how we simplify spawning:

```rust
type Task = async_task::Task<()>;

fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (task, handle) = async_task::spawn(future, |t| QUEUE.send(t).unwrap(), ());
    task.schedule();
    Box::pin(async { handle.await.unwrap() })
}
```

The [`async_task::spawn()`](https://docs.rs/async-task/1.3.0/async_task/fn.spawn.html) constructor takes three arguments:


- The spawned future.
- A schedule function that pushes the task into the queue. This function will be invoked either by the waker or by the `run()` method after polling the future.
- An arbitrary piece of metadata called *tag* that is kept inside the task. Let’s not worry about it in this blog post and simply store `()` as the tag, i.e. nothing.

The constructor then returns two values:

- An `async_task::Task<()>`. The `()` type is the tag.
- An `async_task::JoinHandle<R, ()>`. Again, the `()` type is the tag. This join handle is a `Future` that outputs `Option<R>`, where the output of `None` indicates the task has panicked or got cancelled.

If you're wondering about the [`schedule()`](https://docs.rs/async-task/1.3.0/async_task/struct.Task.html#method.schedule) method, it just invokes the schedule function on the task to push it into the queue. We could've also pushed the `task` into `QUEUE` by ourselves - the end result is the same.

Putting all pieces together, we end up with this remarkably simple executor:

```rust
static QUEUE: Lazy<channel::Sender<Task>> = Lazy::new(|| {
    let (sender, receiver) = channel::unbounded::<Task>();

    for _ in 0..num_cpus::get().max(1) {
        let receiver = receiver.clone();
        thread::spawn(move || receiver.iter().for_each(|task| task.run()));
    }

    sender
});

type Task = async_task::Task<()>;
type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;

fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (task, handle) = async_task::spawn(future, |t| QUEUE.send(t).unwrap(), ());
    task.schedule();
    Box::pin(async { handle.await.unwrap() })
}
```

The complete code can be found in [`v2.rs`](https://github.com/stjepang/byo-executor/blob/master/examples/v2.rs).

The benefit of using `async_task::spawn()` here is not just simplicity. It is also more efficient than hand-rolling our own `Task` as well as more robust. Just to name one example of robustness, `async_task::Task` drops the future immediately after it completes rather than when all task references get dropped.

In addition to that, `async-task` offers useful features like [tags](https://docs.rs/async-task/1.3.0/async_task/struct.Task.html#method.tag) and [cancellation](https://docs.rs/async-task/1.3.0/async_task/struct.Task.html#method.cancel), but let’s not talk about those today. It’s also worth mentioning `async-task` is a [`#[no_std]`](https://docs.rs/async-task/1.3.0/src/async_task/lib.rs.html#116) [crate](https://docs.rs/async-task/1.3.0/src/async_task/lib.rs.html#116) that can even be used without the standard library.


## Improved JoinHandle

If you look at our latest executor closely, there is one remaining instance of inefficiency - the redundant `Box::pin()` allocation for the join handle.

It’d be great if we could use the following type alias, but we can’t because `async_task::JoinHandle<R>` outputs `Option<R>`, whereas our `JoinHandle<R>` outputs `R`:

```rust
type JoinHandle<R> = async_task::JoinHandle<R, ()>;
```

Instead, let’s wrap `async_task::JoinHandle` into a new `struct` that panics if the task panicked or if it was cancelled:

```rust
struct JoinHandle<R>(async_task::JoinHandle<R, ()>);

impl<R> Future for JoinHandle<R> {
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => Poll::Ready(output.expect("task failed")),
        }
    }
}
```

The complete executor implementation can be found in [`v3.rs`](https://github.com/stjepang/byo-executor/blob/master/examples/v3.rs).


## Handling panics

So far we haven’t really thought much about what happens when a task panics, i.e. when a panic occurs inside an invocation of [`poll()`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html#tymethod.poll). Right now, the `run()` method simply propagates the panic into the executor. We should think whether this is what we really want.

It’d be wise to handle those panics somehow. For example, we could simply ignore panics and move on. That way, they are still printed on the screen but won’t crash the whole process — panicking threads work exactly the same way.

To ignore panics, we wrap `run()` into [`catch_unwind()`](https://doc.rust-lang.org/nightly/std/panic/fn.catch_unwind.html):

```rust
use std::panic::catch_unwind;

static QUEUE: Lazy<channel::Sender<Task>> = Lazy::new(|| {
    let (sender, receiver) = channel::unbounded::<Task>();

    for _ in 0..num_cpus::get().max(1) {
        let receiver = receiver.clone();
        thread::spawn(move || {
            receiver.iter().for_each(|task| {
                let _ = catch_unwind(|| task.run());
            })
        });
    }

    sender
});
```

The complete executor that ignores panics be found in [`v4.rs`](https://github.com/stjepang/byo-executor/blob/master/examples/v4.rs).

There are many sensible panic handling strategies. Here are some strategies provided as examples in `async-task`’s repository:

- [Ignore panics](https://github.com/async-rs/async-task/blob/7f91c126df790902636d61362a093871e0d68f77/examples/spawn.rs#L29) — panics are simply ignored and `JoinHandle<R>` panics when awaited.
- [Propagate panics](https://github.com/async-rs/async-task/blob/master/examples/panic-propagation.rs#L64) — panics are re-thrown into the task that awaits the `JoinHandle<R>`.
- [Output panics](https://github.com/async-rs/async-task/blob/master/examples/panic-result.rs#L39) — the `JoinHandle<R>` outputs [`std::thread::Result<R>`](https://doc.rust-lang.org/nightly/std/thread/type.Result.html).

It’s easy to implement any kind of panic handling strategy you want. And it’s totally up to you to decide which one is best!


## How fast is this executor?

The current code is short, simple, and safe. But how fast is it?

A task allocated by `async_task::spawn()` is just a single allocation storing the task state, the future, and the output of the future when it completes. There are no other hidden costs — spawning is virtually as fast as it can possibly be!

Other executors like `async-std` and `tokio` allocate tasks exactly the same way. The basis for our executor is essentially an optimal implementation, and now we’re just one step away from being competitive with popular executors: work stealing.

Right now, all our executor threads share the same task queue. If all threads are hammering the queue at the same time, performance will suffer due to contention. The idea behind work stealing is to assign a distinct queue to each executor thread. That way, an executor thread only needs to steal tasks from other queues when its own queue is empty, meaning contention occurs only rarely rather than all the time.

I’ll talk more about work stealing in another blog post.


## Correctness

*Concurrency is hard*, everybody is telling us. The Go language provides a built-in race detector, `tokio` has [created its own](https://tokio.rs/blog/2019-10-scheduler/#fearless-unsafe-concurrency-with-loom) concurrency checker, [`loom`](https://github.com/tokio-rs/loom), to look for concurrency bugs, and `crossbeam` has in some cases even resorted to formal proofs. Sounds scary!

But we can just sit back, relax, and not worry about it. None of the race detectors, sanitizers, or even [`miri`](https://github.com/rust-lang/miri) or `loom`, can catch bugs in our executor. The reason is that we have only written safe code, and safe code is memory safe, i.e. it can’t contain data races. Rust’s type system has already proven our executor correct.

The burden of ensuring memory safety is entirely on the dependencies, more specifically `async-task` and `crossbeam`. Rest assured, both take correctness very seriously. `async-task` has an [extensive test suite](https://github.com/async-rs/async-task/tree/master/tests) covering all the edge cases,  `crossbeam`'s channel has [lots of tests](https://github.com/crossbeam-rs/crossbeam/tree/ebecb82c740a1b3d9d10f235387848f7e3fa9c68/crossbeam-channel/tests) and even passes the [Go](https://github.com/crossbeam-rs/crossbeam/blob/ebecb82c740a1b3d9d10f235387848f7e3fa9c68/crossbeam-channel/tests/golang.rs) and [`std::sync::mpsc`](https://github.com/crossbeam-rs/crossbeam/blob/ebecb82c740a1b3d9d10f235387848f7e3fa9c68/crossbeam-channel/tests/mpsc.rs) test suites, work-stealing deque is based on a [formally proven](https://fzn.fr/readings/ppopp13.pdf) implementation, while epoch-based garbage collector has a [proof of correctness](https://github.com/crossbeam-rs/rfcs/blob/master/text/2017-07-23-relaxed-memory.md).


## Executors are for everyone

Ever since Alex and Aaron first [designed zero-cost futures](https://aturon.github.io/blog/2016/08/11/futures/) in 2016, the plan was for each spawned future to incur the cost of just a single allocation:


> There is one allocation needed per “task”, which usually works out to one per connection.

However, single-allocation tasks were a white lie — it took us *years* till we actually got them. Consider that `tokio` 0.1 spawns by [allocating the future](https://github.com/tokio-rs/tokio/blob/a05e5a7723df5296284289505d645ee3efd3b502/tokio-threadpool/src/task/mod.rs#L79), then [allocating task state](https://github.com/tokio-rs/tokio/blob/a05e5a7723df5296284289505d645ee3efd3b502/tokio-threadpool/src/sender.rs#L168), and finally [allocating a oneshot channel](https://github.com/tokio-rs/tokio/blob/a05e5a7723df5296284289505d645ee3efd3b502/tokio-threadpool/src/thread_pool.rs#L124). That’s *three* allocations per spawn!

Then, [in August 2019](https://github.com/async-rs/async-task/releases/tag/v1.0.0), `async-task` was [announced](https://async.rs/blog/announcing-async-std/). For the first time ever, we managed to squash the future, task state, and a channel into just a single allocation. The reason why it took us so long is because manual allocation and managing state transitions inside tasks is [incredibly complicated](https://github.com/async-rs/async-task/tree/master/src). But now that it’s been done, you don’t have to worry about any of it ever again.

Soon after that, in [October 2019](https://github.com/tokio-rs/tokio/pull/1657), `tokio` also [adopted the same approach](https://tokio.rs/blog/2019-10-scheduler/#reducing-allocations) with an [implementation](https://github.com/tokio-rs/tokio/tree/1c117fb7fd3ca398ca53ff676485b12cbb08557a/tokio-executor/src/task) similar to `async-task`.

These days, anyone can [trivially](https://github.com/async-rs/async-task/blob/master/examples/spawn.rs) build an efficient executor with single-allocation tasks. What used to be rocket science now isn’t anymore.
