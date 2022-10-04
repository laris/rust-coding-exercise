---
layout: post
title:  "Blocking inside async code"
---

Hi everyone, I haven’t blogged in a while so it feels good to be back.
First things first — here’s some quick news. After two years of work on
[Crossbeam](https://github.com/crossbeam-rs/crossbeam), in 2019 I’ve shifted
my main focus onto asynchronous programming to research the craft of building
runtimes (think of `async-std` and `tokio`). In particular, I want to make
async runtimes more **efficient** and **robust**, while at the same time
also **simpler**.

In this blog post, I’d like to talk a bit about an interesting problem all
runtimes are facing: calling blocking functions from async code.

## Async and sync

We finally have async/await in stable Rust and are now ready to rewrite all
sync code ever written and make it async! Or are we? Should we even do that?
I don’t know.

There’s a growing split between sync and async libraries, examples being the
[`std`](https://doc.rust-lang.org/std/) and
[`async-std`](https://docs.rs/async-std), which look very similar except one has
blocking functions (sync) and the other has non-blocking functions (async).
We also have similar-looking libraries like [`surf`](https://docs.rs/surf) and
[`attohttpc`](https://github.com/sbstp/attohttpc): both are HTTP clients except
one is async and the other is sync. Authors of new libraries are now facing a
dilemma: should they offer a sync or an async API? Should they perhaps offer both?

This API duplication seems very unfortunate right now but I’m optimistic and
believe we’ll eventually figure out how to deal with it. In any case, we need
to find ways to make integration of async and sync code as seamless as possible.

## From sync to async and back

The `main` function in Rust is synchronous so in order to enter the “async world”
from it, we need to do so explicitly. With `async-std`, we enter the async world
by invoking the `block_on()` function:

```rust
use async_std::task;

// This is sync code.
fn main() {
    task::block_on(foo());
}

// This is async code.
async fn foo() {}
```

Now let’s go in the opposite direction and call sync code from async code:

```rust
// This is async code.
async fn foo() {
    bar();
}

// This is sync code.
fn bar() {}
```

So in order to go from async to sync, we don’t have to do any extra steps - we
just invoke the sync function and that’s it! Easy. Except... we need to be
careful about sync functions that take a long time to complete. We cannot call
any sync code from the async world without giving it a second thought.

## Blocking hurts concurrency

One of the core assumptions under which async runtimes operate says every time
a future is polled, it will return either `Ready` or `Pending` very **quickly**.
Blocking for a long time inside async code is a big no-no and should never happen.

To see why, let’s write a program that fetches 40 web pages concurrently using
[`surf`](https://docs.rs/surf):

```rust
use async_std::task;
use std::time::Instant;

// Fetch the HTML contents of a web page.
async fn get(url: &str) -> String {
    surf::get(url).recv_string().await.unwrap()
}

fn main() {
    task::block_on(async {
        let start = Instant::now();
        let mut tasks = Vec::new();

        // Fetch the list of contributors for the first 40 minor Rust releases.
        for i in 0..40 {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0/", i);

            // Spawn a task fetching the list.
            tasks.push(task::spawn(async move {
                let html = get(&url).await;

                // Display the number of contributors to this Rust release.
                for line in html.lines() {
                    if line.contains("individuals") {
                        println!("{}", line.trim());
                    }
                }
            }))
        }

        // Wait for all tasks to complete.
        for t in tasks {
            t.await;
        }

        // Display elapsed time.
        dbg!(start.elapsed());
    });
}
```

The program completes in around 1.5 seconds on my machine. Note that since the
`get` function is asynchronous, we are fetching all 40 web pages concurrently.

Now let’s make the `get` function fetch web pages in a blocking manner. We’ll
replace `surf` with [`attohttpc`](https://github.com/sbstp/attohttpc), which is
a very similar crate, except it offers a sync interface:

```rust
async fn get(url: &str) -> String {
    attohttpc::get(url).send().unwrap().text().unwrap()
}
```

Unsurprisingly, the program is now less efficient and completes in 3 seconds.
My computer has 8 logical cores, which means the `async-std` executor spawns 8
worker threads, and therefore we can only fetch 8 web pages at a time.

The lesson of this exercise is: **blocking hurts concurrency**. It’s really
important that we don’t block inside async code, or else the executor won’t be
able to do useful work — instead, it will just waste time blocking.

## Blocking is everywhere

We saw how blocking inside async code can hurt performance. Granted, that example
was a bit contrived because, of course, you would just use `surf` instead
`attohttpc` and the problem is solved. But the bad news is that blocking is
subtle: it’s **everywhere** and it’s often there without you even being aware of it!

Consider **standard input** and **output**. It’s pretty obvious that reading from
the standard input blocks and you shouldn’t use `std::io::Stdin` inside async code.
But would you raise a brow if you saw `println!()` there? I bet we all most of the
time assume printing to standard output does not block while it really could.

In case you’re wondering why `println!()` can block, imagine we executed
`program1 | program2` in a shell so that the output of `program1` is piped
into `program2`. If `program2` is reading input very slowly, then `program1`
will have to block whenever it prints something and the pipe is full.

Intensive **computation** can also block. Imagine we’re sorting a really big
`Vec` by calling `v.sort()`. If sorting takes a second or so to complete, we
should really consider moving that computation off the async executor.

Sometimes there are even “traps” a programmer that is not too careful might
fall into. For example, suppose we use Rayon to call `v.par_sort()` inside
async code. It might be tempting to naively think this is okay because sorting
happens inside Rayon’s executor, while the truth is the async executor will
still block to wait for Rayon’s result.

But lower performance is not the only concern. If every thread of the async
executor gets stuck on something like reading from the standard input, it’s
also possible for the whole program to get into a **deadlocked** state, unable
to make any kind of progress!

Finally, it’s worth adding that even plain memory access can be blocking! As
an example, consider swap space that resides on a spinning disk. If a
thread is accessing swap space that is currently on the disk, it will have
to block until that page is fetched from the physical disk and moved into
the main memory.

So blocking is really **pervasive** and it’s difficult to untangle it
completely from async code. I believe we just have to embrace the reality
that blocking will always be inside async code, no matter how carefully we
try to eliminate it.

## Possible solutions

When we anticipate blocking inside async code, we should consider offloading
the blocking logic onto a different thread pool so that the executor can keep
running without having to wait for it. Runtimes like `async-std` and `tokio`
provide the **[`spawn_blocking()`](https://docs.rs/async-std/1.2.0/async_std/task/fn.spawn_blocking.html)**
function to help with that.

To illustrate how the function is used, let’s take a look at how
[`fs::read_to_string()`](https://docs.rs/async-std/1.2.0/async_std/fs/fn.read_to_string.html)
is implemented inside `async-std`:

```rust
async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || std::fs::read_to_string(path)).await
}
```

Function `spawn_blocking()` spawns the closure onto a special thread pool that is dedicated to running blocking functions. The async executor then doesn’t have to block on closure’s result and instead `await`s the result from the returned `JoinHandle` asynchronously.

Note that we cannot pass a reference to `path` into the closure because the async `read_to_string()` function might be canceled before the sync version completes. Unfortunately, the only way of passing `path` into the closure is by cloning it. This is somewhat inefficient and also feels a bit clunky.

Fortunately, Tokio has an alternative way of running blocking functions: it can execute the closure in-place and tell the current thread to stop being a part of the async executor and hand over that responsibility to a new thread. In a way, it’s the opposite of `spawn_blocking()` — instead of sending the closure to a new thread and continuing the event loop, we send the event loop to a new thread and continue running the closure.

This is how **[`block_in_place()`](https://docs.rs/tokio/0.2.2/tokio/task/fn.block_in_place.html)**
could be used to implement an async version of `read_to_string()`:

```rust
async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    block_in_place(|| std::fs::read_to_string(path))
}
```

Note that we don’t have to clone `path` anymore because it’s impossible to cancel the outer async `read_to_string()` before the inner sync `read_to_string()` completes.

While both `spawn_blocking()` and `block_in_place()` solve the problem of async executor getting stuck on blocking code, there’s an important difference between them. Note that `spawn_blocking()` is really an async function because it returns a future that can be awaited, while `block_in_place()` is just a regular sync function.

To see why that matters, consider the following:

```rust
let (s1, s2) = futures::join!(read_to_string("foo.txt"), read_to_string("bar.txt"));
```

If `read_to_string()` is implemented using `spawn_blocking()`, the two files will be read **in parallel**. However, if it is implemented using `block_in_place()`, they will be read **sequentially** one after another!

## Conclusion

The key takeaways are:

- Blocking inside async code will make performance suffer or even cause deadlocks.
- We need to isolate blocking parts of the program using `spawn_blocking()` or
  `block_in_place()`.
- Blocking is pervasive and it’s hard to isolate it completely.

Furthermore, sometimes it’s hard to even say what code is blocking and what code isn’t.
If a function takes 1 second to complete, we probably consider it a blocking one. But
what if it takes 1 millisecond? Well, depends on the particular use case — sometimes
we should consider that blocking and sometimes we shouldn't. It really depends!

Blocking is scary and we need to defensively isolate it from async code.
But there’s only so much we can do, and blocking will still inevitably creep
into our async code. This might sound like a sad and disappointing state of affairs,
but I’m optimistic. I believe there are better solutions than `spawn_blocking()`
and `block_in_place()`, which I’ll talk about in a following blog post.
