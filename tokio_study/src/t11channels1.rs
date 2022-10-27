/*
use mini_redis::client;

#[tokio::main]
async fn main() {
    // Establish a connection to the server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async {
        let res = client.get("hello").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
*/
// define the message type
use bytes::Bytes;
#[derive(Debug)]
enum Command {
    Get { key: String, },
    Set { key: String, val: Bytes, },
}
/*
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create a new channel with capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);
    // Rest comes here
}
*/
use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("Sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("Sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
