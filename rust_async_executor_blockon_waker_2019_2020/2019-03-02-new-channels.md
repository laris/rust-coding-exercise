---
layout: post
title:  "Proposal: New channels for Rust's standard library"
---

Two exciting performance improvements are coming to Rust's standard library soon -
we're replacing mutexes with [`parking_lot`](https://github.com/rust-lang/rust/pull/56410)
and replacing hash maps with [`hashbrown`](https://github.com/rust-lang/rust/pull/56241).
The public interface will stay the same while the internal implementations are swapped
out for much faster ones.
All Rust programs using those primitives will therefore magically get faster, too!

In this blog post, I'm proposing we also replace the guts of
[`mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html) with
[`crossbeam-channel`](https://docs.rs/crossbeam-channel) for some more performance wins.
However, unlike with mutexes and hash maps, this change will also enable oft-requested new
features that make it tempting to deprecate `mpsc` altogether and introduce better
channels designed from scratch.

The `mpsc` channels are not perfect. We have
[a bunch of regrets](https://github.com/rust-lang/rust/pull/42397#issuecomment-315867774)
over some decisions made before their stabilization. Now could be a good time to
revisit those decisions and consider fixing the mistakes.

## The dilemma 

Just a quick reminder worth bringing up first. Unstable feature 
[`mpsc_select`](https://github.com/rust-lang/rust/issues/27800)
was introduced in 2015 and we've decided to deprecate it in late 2018 with the
intention of removing in a future Rust release.
The reason is that it adds a lot of complexity that feels like
shouldn't belong to the standard library. The feature never worked
as well as we hoped and `crossbeam-channel` is a better alternative anyway.
I fully support this decision
as selection makes channels at least 2x more complex when measured in lines of
code, and users who need selection can always reach for `crossbeam-channel`.

We probably don't want to copy the whole `crossbeam-channel` into `std`
because it is just way too big. Without tests, comments, and blank lines,
it contains ~4100 lines of code, while `mpsc` contains ~2300, and even that is plenty.
We'd instead prefer a slimmer version of `crossbeam-channel` that doesn't support
selection. I have a tentative new implementation for `mpsc`
based on `crossbeam-channel` that is only ~1700 lines long. You can find it
[here](https://github.com/stjepang/new-channel), but keep in mind it's still a work
in progress.

I'm sure everyone wants `mpsc` to be faster because why not. However, there might be
disagreements on how far to take this proposal - besides improving performance,
we could also improve the interface and enable new features:

1. One option is uncontroversial: swap the codebase behind `mpsc` and
   finally implement `Sync` for `Sender`.

2. The other option is more radical: introduce `std::sync::channel` module
   with the new implementation and a modern API. `std::sync::mpsc` then becomes obsolete so
   we'd probably want to deprecate it at some point.

While the second option might seem a bit too far-reaching, I'll try to make a
convincing case for it and demonstrate the benefits would be worth it.

The summary of the argument is the following. The whole `mpsc` module is unnecessarily
complicated to use, uses odd jargon, is generally a poor example of design in Rust,
and if we were to do it from scratch today, we'd do it much differently. I personally
even feel it'd be better to have *no* channels than keep `mpsc` as is - it is that bad!

## Why have channels in `std` at all?

It has sometimes been suggested we deprecate `mpsc` and point users to external channel
crates like `crossbeam-channel`. While this is a compelling option, I think in 2019
channels are a fundamental synchronization primitive, and we do need them in the
standard library. They are basically as important as mutexes! Every modern programming
language should have at least very basic channels in its concurrency kit.

And if we're going to have channels in the standard library, what should be the
difference between those and channels in external crates? My position is the
standard library's channels should focus on:

* Simplicity. The interface must be lean and simple, and the codebase must be
  understandable. We don't want crazy optimizations that make the codebase
  too challenging to maintain.

* Good performance. It doesn't have to be best-in-class but has to be reasonable.
  Just wrapping a `VecDeque` inside a `Mutex` to make the queue concurrent
  would be disappointing.

* Fast compilation. The current implementation monomorphizes so much code you
  can notice `mpsc` increasing compilation times!

## The "no-brainer" proposal

If we're to keep `mpsc` as the channel module in `std`, then we should at
least incorporate performance improvements from `crossbeam-channel` and
implement `Sync` for `Sender`.

More concretely, this is the conservative proposal I don't expect anyone to
have objections to:

1. Delete the unstable and deprecated `mpsc_select` feature.
2. Swap out the guts of `mpsc` with the slimmed-down version of `crossbeam-channel`.
3. Add `unsafe impl<T: Send> Sync for Sender<T> {}`.
4. Do a crater run to make sure there are no regressions.

If we go this route, the benefits will be:

* Faster channels overall. In particular, bounded channels become *much* faster.
* `Sender` finally implements `Sync`.
* A long-standing [bug](https://github.com/rust-lang/rust/issues/39364) gets fixed.
* Fewer `unsafe` blocks - we go from 101 down to 33.
* Shorter compilation time for code using channels - a *hello world* with channels goes
  from 1.6 sec down to 1.2 sec on my machine.

There will be some drawbacks, too, but they're relatively minor:

* Bounded channels use more memory - every slot in the buffer contains an
  additional `AtomicUsize`.

* Unbounded channels are slower to construct and send a single message.
  Creating a channel, sending a message, receiving a message,
  and dropping the channel goes from 83 ns to 265 ns.
  The current `mpsc` implementation has a special optimization for "oneshot" channels
  that can't be used anymore. Unfortunately, this optimization also prevents `Sender`
  from implementing `Sync`.

## We can have multiple consumers now!

The new implementation will incidentally also make it possible to implement `Sync`
and `Clone` for `Receiver` trivially. Currently, if one wants to share the receiving side
of a channel, they have to jump through hoops.

You can see this in the
[Graceful Shutdown and Cleanup](https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html)
chapter of The Book: the receiving side is of type `Arc<Mutex<mpsc::Receiver<Job>>>` and
we receive messages with `receiver.lock().unwrap().recv().unwrap()`. This is not pretty
code at all but makes sense since the "SC" part in "MPSC" stands for *single-consumer*.

But why is `mpsc` a single-consumer channel anyway? Why didn't we go with
multi-consumer channels from the beginning? I haven't been involved with Rust
at the time so cannot be completely sure about the real reason, but I believe it boils down
to fast unbounded multi-consumer channels being difficult to implement without
garbage collection.

Unbounded channels have to be represented as a linked list, so receive operations
need to load the head node of the list and do a compare-and-swap to change the head
pointer to the next node. But reading the next pointer in the head node is dangerous
if there's a concurrent receive operation that might pop this node and deallocate it!

If we were to write a multi-consumer channel in Java or Go, we'd say "whatever" and let
the GC handle deallocation. But Rust offers no such luxury. The usual solution
for us is to use epoch-based garbage collection, but it's a too big and complex piece 
of code to have in the standard library.

Fortunately, it's possible to implement a multi-consumer channel with a relatively simple
[trick](https://github.com/crossbeam-rs/crossbeam/pull/279#issuecomment-450490718)
that allocates nodes in segments and occasionally locks
segments for a very short time. This keeps good scalability of channels, simplifies
the code, and even makes it faster in the typical case due to smaller overhead incurred
by GC-related book-keeping. But this trick wasn't well-known when `mpsc` was created.

## The API needs improvement

Let's take a good look at the current API for `mpsc` channels. I'll omit iterators
and error types because we designed them right the first time and they're uninteresting
for the sake of this analysis.

There are three types that represent channels and two constructors:

```rust
struct Sender<T> {}
struct SyncSender<T> {}
struct Receiver<T> {}

fn channel<T>() -> (Sender<T>, Receiver<T>);
fn sync_channel<T>(n: usize) -> (SyncSender<T>, Receiver<T>);
```

Some of those types implement `Clone`, `Send`, and `Sync`:

```rust
impl<T> Clone for Sender<T> {}
impl<T: Send> Send for Sender<T> {}

impl<T> Clone for SyncSender<T> {}
impl<T: Send> Send for SyncSender<T> {}
impl<T: Send> Sync for SyncSender<T> {}

impl<T: Send> Send for Receiver<T> {}
```

Finally, methods for sending and receiving messages:

```rust
impl<T> Sender<T> {
    fn send(&self, t: T) -> Result<(), SendError<T>>;
}

impl<T> SyncSender<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>>;
    fn send(&self, t: T) -> Result<(), SendError<T>>;
}

impl<T> Receiver<T> {
    fn try_recv(&self) -> Result<T, TryRecvError>;
    fn recv(&self) -> Result<T, RecvError>;
    fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>;
}
```

Note how `SyncSender` is strictly more powerful than `Sender` - it has a
superset of features. And really, there is no good reason why we need two distinct
sender types and a single receiver type. If we were to design channels from scratch
today, I'm sure there would be just a single `Sender` type.

As already mentioned, another issue with this API is the fact that `Sender`
doesn't implement `Sync`, which will be fixed by the new channel
implementation.

The third issue is the lack of `send_timeout` method on `SyncSender`, which would
block for a limited time when the channel is full. If `Receiver` has `recv_timeout`,
why wouldn't there be a similar method on the sending side? I believe the omission
of this method is just an oversight and we don't have it only because nobody has
implemented it yet.

## Confusing terminology

Here's a line of code from The Book, chapter
[Message Passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html),
under heading *Creating Multiple Producers by Cloning the Transmitter*:

```rust
let tx1 = mpsc::Sender::clone(&tx);
```

Okay, so we're creating a new *transmitter* by cloning the *sender*
side of a multi-*producer*... wait, that's three different words describing the
same concept in a single line!

Synonyms like that make it for more painful user experience than it has to be,
and we're just getting started. Let's take a complete tour through all the issues
of the terminology used by `mpsc`.

### Synonyms, shorthands, acronyms

Is it *sender*, *transmitter*, or *producer*? Is it *receiver* or *consumer*?
To make matters worse, in Servo, *senders* are called *chans* and
*receivers* are called *ports*. That's a lot of words to remember.

The shorthands for *sender* and *receiver* are *tx* and *rx*. Why not
just use *s* and *r* instead? When I was learning Rust, I had to
google for "tx", read an [article](https://en.wikipedia.org/wiki/TX) on Wikipedia to
learn it stands for *transmission* in telecommunications, and then connect the dots
to realize it's a synonym for *sender*.

And what about the cryptic name `mpsc`? I can't count how many times I saw it
misspelled as `mspc` or something of that sort. It's not an acronym that
rolls off the tongue nor is it easy to remember. We could've named the module
`chan` or `channel` instead.

### Synchronous vs asynchronous

There are three types of channels:

* Unbounded channels - they don't have a fixed capacity.
* Bounded channels - they have a fixed capacity.
* Zero-capacity channels - the capacity is zero (a special case of bounded channels).

Now let's see what these three types are called in various libraries.

|   | Unbounded | Bounded | Zero-capacity |
|---|-----------|---------|---------------|
| Golang | N/A | asynchronous/buffered | synchronous/unbuffered |
| `std::sync::mpsc` | asynchronous | synchronous | rendezvous |
| `futures-channel` | unbounded | bounded | N/A | 
| `crossbeam-channel` | unbounded | bounded | zero-capacity |

I think the most frustrating part here is that what `mpsc` calls *synchronous*,
Golang calls *asynchronous*. Oof. The logic is probably in that `mpsc` thinks of
bounded channels as
*synchronous when full* and Golang thinks of them as *asynchronous when not full*.
Both of them are correct in their own ways, but this is a real mess.

We're about to get async/await soon, so I expect we'll start talking about asynchronous
channels in a completely different context, which will make the confusion even worse.

And we're not done yet! Note that when we, e.g. say `SyncSender` implements `Sync`, trait
`Sync` has absolutely nothing to do with the prefix `Sync`. The `Sync` trait means
it can be shared by reference across threads, while the `Sync` prefix means it's
a bounded channel.

We should've called channels *bounded* and *unbounded* instead. That way, there'd
be no chance of mistaking those words for anything else.

### Disconnect vs close

When all senders or all receivers associated with a channel get dropped, the channel
becomes *disconnected*, meaning no more messages can be sent into it. I think use
of the word *disconnected* is perfectly fine here, but it's unfortunate how pretty much
everywhere else channels get *closed* instead. While this may seem like a minor
annoyance at worst, it's becoming more and more of a problem.

First of all, within the codebase of `mpsc`, channels get *closed*. Only in the
public documentation they get *disconnected*.

In `crossbeam-channel`, I use *disconnected* just because `mpsc` uses the same,
but I'm seriously considering switching to *closed* before publishing version 1.0.

In [Unix pipes](http://tldp.org/LDP/lpg/node11.html),
[`ipc-channel`](https://docs.rs/ipc-channel/0.11.3/ipc_channel/ipc/enum.IpcSelectionResult.html#variant.ChannelClosed),
[Go channels](https://gobyexample.com/closing-channels),
[proposed C++ queues](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p0260r3.html#closed_queues),
and [Node.js streams](https://nodejs.org/api/stream.html#stream_event_close),
*closed* is used.

In [`futures-channel`](https://docs.rs/futures-channel-preview/0.3.0-alpha.13/futures_channel/mpsc/struct.Sender.html#method.disconnect),
channels get *closed*, but senders and receivers can also be manually *disconnected*,
which essentially means handles become "null" and using them will result in a panic.
Therefore, *disconnect* in
`mpsc` and in `futures-channel` does not mean the same thing at all, which is really confusing!

Finally, *disconnected* is a bit longer than *closed* - consider
`RecvTimeoutError::Disconnected`. That's wordy and not fun to type.

## The "clean slate" proposal

If we're going to introduce new channels rather than try fixing `mpsc`, let's
revamp the interface entirely and avoid all the mistakes previously made.

I suggest we take the following steps in that case:

1. Create `std::sync::channel` module with new channels.
2. Change the guts of `mpsc` to use `channel` behind the scenes, but otherwise don't change it.
3. Do a crater run to make sure there are no regressions.
4. Stabilize `std::sync::channel`.
5. Phase out `mpsc` and nudge users towards `channel`.

Here's how the new channels would look. There is only a single sender type
for both bounded and unbounded channels:

```rust
struct Sender<T> {}
struct Receiver<T> {}

fn unbounded<T>() -> (Sender<T>, Receiver<T>);
fn bounded<T>(n: usize) -> (Sender<T>, Receiver<T>);
```

`Sender` and `Receiver` implement all of `Clone`, `Send`, and `Sync`, so they
can be shared across threads in any way you find most convenient:

```rust
impl<T> Clone for Sender<T> {}
impl<T: Send> Send for Sender<T> {}
impl<T: Send> Sync for Sender<T> {}

impl<T> Clone for Receiver<T> {}
impl<T: Send> Send for Receiver<T> {}
impl<T: Send> Sync for Receiver<T> {}
```

The methods for sending and receiving messages come in three flavors: non-blocking,
blocking, and blocking with a timeout.

```rust
impl<T> Sender<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>>;
    fn send(&self, t: T) -> Result<(), SendError<T>>;
    fn send_timeout(&self, t: T, timeout: Duration) -> Result<(), SendTimeoutError<T>>;
}

impl<T> Receiver<T> {
    fn try_recv(&self) -> Result<T, TryRecvError>;
    fn recv(&self) -> Result<T, RecvError>;
    fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>;
}
```

Note how beautifully symmetrical `Sender` and `Receiver` now are. And this API is more
powerful than `mpsc` despite being smaller and simpler!

The new channels would use clearer terminology:

* There's no mention of *producers*, *consumers*, or *transmitters*.
* Also no mention of *synchronous* and *asynchronous* channels - they are *bounded* and
  *unbounded* instead.
* Senders and receivers are abbreviated as *s* and *r*.
* Channels get *closed* rather than *disconnected*.

If you'd like to see the full interface with examples, check out the
[documentation](https://stjepang.github.io/doc/new_channel/index.html)
generated from the prototype and compare it to the
[documentation](https://doc.rust-lang.org/std/sync/mpsc/index.html)
for `mpsc` channels.

## Conclusion

Despite all the flaws, the `mpsc` module is a brilliant piece of code
and was one of the coolest and most advanced channel implementations at the time
Rust 1.0 came out. But state of the art has progressed since then and I believe
it's time for it to go.

Our terminology around `mpsc` channels is all over the place. It's a hindrance to
learning, and I think the unnecessary clunkiness paints a bad picture of Rust.
Now is a great chance to come up with a new
and well-thought-out language for talking about channels that will be adopted throughout
the library ecosystem. Currently, every channel library has its own set of
annoying inconsistencies deviating from others, making the whole situation even worse.

I'm aware adding new APIs and removing old ones from the standard library is going to
be painful. But my opinion is keeping the status quo is the worse scenario and
being stuck with poorly designed channels will be even more painful down the road.
I hope we fix the mistakes in our channels and the sooner that happens, the better.

If we decide to transition to the new `channel` module, I promise to help by:

* Writing clear instructions in the docs on how to switch from `mpsc` to `channel`.
* Refreshing The Book with new idioms. Do we accept pull requests?
* Updating the Rust Cookbook.
* Pushing all channel libraries to follow suit by using consistent naming with the
  new channels.

And, of course, if we decide to stick with `mpsc`, I'll still swap the codebase for the new one
and add the missing features like `Sync` implementation for `Sender` and the
`send_timeout` method.
