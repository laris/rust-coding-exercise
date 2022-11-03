---
layout: post
title:  "Build your own block_on()"
---

If you’ve ever wondered how [`block_on`](https://docs.rs/futures/0.3.1/futures/executor/fn.block_on.html) from the [`futures`](https://docs.rs/futures) crate works, today we are going to write our own version of the function.

Inspiration for this blog post comes from two crates, [`wakeful`](https://docs.rs/wakeful) and [`extreme`](https://github.com/spacejam/extreme). `wakeful` has devised a simple way to create a [`Waker`](https://doc.rust-lang.org/nightly/std/task/struct.Waker.html) from a function, while `extreme` is an extremely terse implementation of `block_on()`.

Our implementation will have slightly different goals from `extreme`. Rather than going for zero dependencies and minimal number of lines of code, we’ll go for a safe and efficient but still pretty simple implementation.

Dependencies we’re going to use are [`pin-utils`](https://docs.rs/pin-utils), [`crossbeam`](https://docs.rs/crossbeam), and [`async-task`](https://docs.rs/async-task).

## Function signature

The signature of `block_on` looks as follows. We take a future as an argument, run it on the current thread (blocking whenever it is pending), and return its output:

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    todo!()
}
```

Now let’s implement the missing `todo!()` part…

## First attempt

Note that the [`poll`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html#tymethod.poll) method on every [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html) takes a pinned future. So we need to pin it first. While there is a way to do that safely using [`Box::pin()`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html), we’d rather pin the future on the stack than heap.

Unfortunately, the only way of pinning futures on the stack safely is by using the `pin-utils` crate:

```rust
pin_utils::pin_mut!(future);
```

The [`pin_mut`](https://docs.rs/pin-utils/0.1.0-alpha.4/pin_utils/macro.pin_mut.html) macro converts `future` from a variable of type `F` into one of type `Pin<&mut F>`.

Next we’ll need to specify what happens when this future is woken. In this case, waking should simply unblock the thread running the future.

Constructing a `Waker` can be gnarly — just take a peek at the [implementation](https://docs.rs/extreme/666.666.666666/src/extreme/lib.rs.html) of `extreme`. And this is the simplest possible way of constructing a `Waker` by hand! So many raw pointers, so much unsafe code… let's skip this part for now and fill in the blank later.

```rust
let waker = todo!();
```

Finally, we create a task context from the waker and keep polling the future in a loop. If it completes, return the output. If it’s pending, block the current thread:


```rust
let cx = &mut Context::from_waker(&waker);
loop {
    match future.as_mut().poll(cx) {
        Poll::Ready(output) => return output,
        Poll::Pending => thread::park(),
    }
}
```

In case you're puzzled by the [`Context`](https://doc.rust-lang.org/nightly/std/task/struct.Context.html) type, it’s a wrapper around `Waker` — there’s nothing more to it. When async/await in Rust was being designed, we weren’t sure if it’d be useful to pass anything else besides a `Waker` to `poll()` so we came up with this wrapper that might hold more stuff in a future version of Rust.

Anyways… we’re almost done. Let’s go back to waker construction and fill in the blank marked with `todo!()`.

If you think about it, `Waker` is really just a carefully optimized, fancy version of  `Arc<dyn Fn() + Send + Sync>` , and [`wake()`](https://doc.rust-lang.org/nightly/std/task/struct.Waker.html#method.wake) invokes this function. Also put yet another way, a `Waker` is a callback that gets invoked whenever the future can continue execution.

Since `Waker` is so difficult to construct, [sagebind](https://github.com/sagebind) came up [`waker_fn()`](https://docs.rs/wakeful/0.1.1/wakeful/fn.waker_fn.html), a straightforward way to convert any function into a `Waker`. Unfortunately, `wakeful` seems to be yanked at the moment, so I borrowed `waker_fn()` and put it into my crate `async-task`.

In our `block_on`, the callback unblocks the thread running the future:

```rust
let thread = thread::current();
let waker = async_task::waker_fn(move || thread.unpark());
```

So simple! Much better than fiddling with [`RawWaker`](https://doc.rust-lang.org/nightly/std/task/struct.RawWaker.html) and [`RawWakerVTable`](https://doc.rust-lang.org/nightly/std/task/struct.RawWakerVTable.html).

Internally, the `waker_fn()` constructor literally creates an `Arc<impl Fn() + Send + Sync>` and then converts it into `Waker` with unsafe code that looks similar to what we saw in `extreme`.

Here’s a complete implementation of `block_on()`:

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    let thread = thread::current();
    let waker = async_task::waker_fn(move || thread.unpark());

    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}
```

See [`v1.rs`](https://github.com/stjepang/byo-block-on/blob/master/examples/v1.rs) if you’d like to try running this code.

## A problem with parking

But, it’s not time to celebrate yet. There’s a problem. If user code inside the future also makes use of the park/unpark API, it may pick up and “steal” unpark notifications from the callback. Read [this issue](https://github.com/rust-lang/futures-rs/pull/2010) for a more elaborate explanation.

A possible solution is to use a way of parking and unparking threads different from the one inside the [`std::thread`](https://doc.rust-lang.org/nightly/std/thread/index.html) module. That way, code inside the future will not be able to interfere with waking.

There’s a very similar park/unpark mechanism in `crossbeam`, except it allows us to create arbitrarily many independent [parkers](https://docs.rs/crossbeam/0.7.3/crossbeam/sync/struct.Parker.html) rather than having one per thread. Let’s create one per invocation of `block_on()`:

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || unparker.unpark());

    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}
```

That’s it! Problem solved.

See [`v2.rs`](https://github.com/stjepang/byo-block-on/blob/master/examples/v2.rs) if you’d like to try running this code.

## A caching optimization

Creating a [`Parker`](https://docs.rs/crossbeam/0.7.3/crossbeam/sync/struct.Parker.html) and `Waker` is not free — both of those incur the cost of an allocation, which is unfortunate. Can we improve?

Instead of constructing a `Parker`  and `Waker` on each invocation of `block_on`, why not cache them in thread-local storage? That way a thread will reuse the same instances across all invocations of `block_on()`:

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    thread_local! {
        static CACHE: (Parker, Waker) = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            (parker, waker)
        };
    }

    CACHE.with(|(parker, waker)| {
        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}
```

If the future is quick to execute, this small change will make `block_on()` dramatically more efficient!

See [`v3.rs`](https://github.com/stjepang/byo-block-on/blob/master/examples/v3.rs) if you’d like to try running this code.

## What about recursion?

Are we done yet? Well… just one more last thing.

What if the future inside `block_on()` calls `block_on()` again recursively? We can either permit or forbid recursion.

If we choose to permit recursion, then we also need to make sure recursive calls of `block_on()`  don’t share the same `Parker` and `Waker` instances, or else there’s no way to tell which `block_on()` invocation gets woken.

The `block_on()` from the `futures` crate panics on recursive invocations of `block_on()`. I don’t have a strong opinion on whether permitting or forbidding recursion is better — both behaviors are sensible. But, since we’re mimicking the `futures` version, let’s forbid recursion.

To detect recursive invocations, we could introduce another thread-local variable indicating whether we’re currently inside `block_on()` or not. But that’s a lot of work.

Here’s a cool trick that requires fewer changes to the code. Let’s wrap `(Parker, Waker)` into a [`RefCell`](https://doc.rust-lang.org/nightly/std/cell/struct.RefCell.html), and panic if a mutable borrow is already active:

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    thread_local! {
        static CACHE: RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            RefCell::new((parker, waker))
        };
    }

    CACHE.with(|cache| {
        let (parker, waker) = &mut *cache.try_borrow_mut().ok()
            .expect("recursive `block_on`");

        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}
```

Finally. Now we’re really done, I promise! This final implementation is as correct, as robust, and as efficient as it gets. More or less. :)

See [`v4.rs`](https://github.com/stjepang/byo-block-on/blob/master/examples/v4.rs) if you’d like to try running this code.

## Benchmarks

To test how efficient our `block_on()` is, let’s benchmark it against the one from `futures`.

But first, we’ll write a helper future type that yields a number of times and then completes:

```rust
struct Yields(u32);

impl Future for Yields {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

As an example, to benchmark a future yielding 10 times, we write:


```rust
#[bench]
fn custom_block_on_10_yields(b: &mut Bencher) {
    b.iter(|| block_on(Yields(10)));
}
```

Let’s make a set of three benchmarks with futures yielding 0, 10, and 50 times. We run those using our custom `block_on()` and then using `block_on()` from `futures`. You can find the full benchmark code in [`yield.rs`](https://github.com/stjepang/byo-block-on/blob/master/benches/yield.rs).

And here are the results on my machine:

```
test custom_block_on_0_yields   ... bench:           3 ns/iter (+/- 0)
test custom_block_on_10_yields  ... bench:         130 ns/iter (+/- 12)
test custom_block_on_50_yields  ... bench:         638 ns/iter (+/- 20)
```

```
test futures_block_on_0_yields  ... bench:          10 ns/iter (+/- 0)
test futures_block_on_10_yields ... bench:         236 ns/iter (+/- 10)
test futures_block_on_50_yields ... bench:       1,139 ns/iter (+/- 30)
```

The numbers say our custom `block_on()` is roughly 2 or 3 times faster in this particular benchmark, which is not bad at all!

## Conclusion

Async Rust can feel intimidating because it contains so much machinery: the `Future` trait, pinning, the `Context` type, `Waker` and its friends `RawWaker` and `RawWakerVTable`, desugaring of `async` and `await`, unsafe code, raw pointers, and so on.

But the thing is, a lot of the ugly stuff is not even that important — it’s really just boring boilerplate that can be removed with crates like `pin-utils`, `async-task`, and `crossbeam`.

And indeed, today we managed to build an efficient `block_on()` in few lines of safe code without having to understand most of that boilerplate. In another blog post, we’ll build a real executor…
