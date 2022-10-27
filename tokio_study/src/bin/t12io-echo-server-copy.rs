use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("listener accept() action!");
        println!("{:?}", &socket);

        tokio::spawn(async move {
            println!("tokio::spawn async move action!");
            // copy data here
            let (mut rd, mut wr) = socket.split();
            println!("{rd:?}");
            println!("{wr:?}");
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
