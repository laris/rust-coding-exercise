use tokio::task::yield_now;
use std::rc::Rc;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        //--------------------------------------------------------------------------------
        // The scope forces `rc` to drop before `.await`.
        // { let rc = Rc::new("hello");
            // println!("{}", rc); }
        // `rc` is no longer used. It's **not** persisted when
        // the task yields to the scheduler
        // yield_now().await;
        //--------------------------------------------------------------------------------
        // let rc = Rc::new("hello");
        // yield_now().await;
        // println!("{}", rc);
        //--------------------------------------------------------------------------------
        let rc = Arc::new("hello");
        yield_now().await;
        println!("{}", rc);
    });
}