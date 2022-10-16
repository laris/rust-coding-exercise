use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;

async fn process(socket: TcpStream) {
    // A hashmap is used to store data
    let mut db = HashMap::new();
    // Connection, provided by `mini-redis`,
    // handles parsing frames from t he socket
    let mut connection = Connection::new(socket);
    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`.
                    // This type will be convered later in the tutorial.
                    // For now, `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else { Frame::Null }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}


#[tokio::main]
async fn main() {
// async fn main() -> Result<(), Box<dyn Error>>{
    // Bind the listener to the addr
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // The second item contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();
        // let (socket, _) = listener.accept().await?;
        process(socket).await;
    }
    // Ok(())
}