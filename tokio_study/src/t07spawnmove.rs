use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    let handle = task::spawn(async move {
        println!("Here's a moved into spawn vec: {:?}", v);
    });

    handle.await;
}