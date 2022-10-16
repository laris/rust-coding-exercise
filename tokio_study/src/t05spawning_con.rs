use tokio::net::TcpListener;
mod t04spawning;
use t04spawning::process;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket.
        // The socket is moved to the new task and processed there
        tokio::spawn(async move {{
            process(socket).await;
        }});
    }
}