use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
// use std::error::Error;

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

// async fn process(socket: TcpStream) -> Result<(), Box<dyn Error>> {
pub async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got frame: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap()
    }
    // Ok(())
}