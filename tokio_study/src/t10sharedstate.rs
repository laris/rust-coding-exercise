#[allow(unused_imports)]
#[allow(dead_code)]
// require bytes crate, Arc<Vec<u8>>
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }

}

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// Shard Mutex
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

/*
fn sharded_db_usage() {
    let shard = db[Hash(key) % db.len()].lock().unwrap();
    shard.insert(key, value);
}
*/

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    // connection, provided by `mini-redis`,
    // handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else { Frame::Null }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

// Holding a MutexGuard across an .await
// use std::sync::{Mutex, MutexGuard};
use std::sync::{MutexGuard};
// compile error, MutexGuard is not Send
async fn increment_and_do_stuff_1(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
    // do_something_async().await;
}   // lock goes out of scope here

// works
async fn increment_and_do_stuff_2(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here
    // do_something_async().await;
}

// fail
async fn increment_and_do_stuff_3(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
    drop(lock);
    // do_something_async().await;
}

// re-structure code to not hold the lock across an .await
struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // this function is not marked async
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}
async fn increment_and_do_stuff_4(can_incr: &CanIncrement) {
    can_incr.increment();
    //do_something_async().await;
}

// Tokio's asynchronous mutex
use tokio::sync::Mutex as TokioMutex;

// This compiles!
// (but restructuring the code would be better in this case)
async fn increment_and_do_stuff(mutex: &TokioMutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;
    //do_something_async().await;
} // lock goes out of scope here
