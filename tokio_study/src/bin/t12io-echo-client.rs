use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);
    println!("{rd:?}"); // split::ReadHalf
    println!("{wr:?}"); // split::WriteHalf

    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        //Sometimes, the rust type inferencer need a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 { break; }
        println!("GOT {:?}", &buf[..n]);
        for c in &buf {
            print!("{}", *c as char);
        }
        println!();
    }
    Ok(())
}
