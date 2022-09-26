// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, async world!");
}

struct Song();
async fn learn_song() -> Song { Song() }
async fn sing_song(song: Song) {}
async fn dance() {}

async fn learn_and_sing() {
   // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}
fn main() {
   // println!("Hello, world!"); 
   let future = hello_world(); // Nothing is printed
//    println!("{:?}", future); // debug future type: impl futures::Future<Output = ()>
   block_on(future); //`future` is run and "hello, world!" is printed

   let song = block_on(learn_song());
   block_on(sing_song(song));
   block_on(dance());
}
