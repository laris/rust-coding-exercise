use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // read()
    let mut f1 = File::open("foo1.txt").await?;
    let mut buf1 = [0; 10];

    // read up to 10 bytes
    let n = f1.read(&mut buf1[..]).await?;
    println!("The bytes: {:?}, via .read()", &buf1[..n]);
    //Ok(())

    // read_to_end()
    let mut f2 = File::open("foo1.txt").await?;
    let mut buf2 = Vec::new();
    // read the whole file
    f2.read_to_end(&mut buf2).await?;
    println!("THe bytes: {:?}, via .read_to_end()", &buf2);
    //Ok(())

    // write()
    let mut f3 = File::create("foo3.txt").await?;
    //Writes some prefix of the byte string, but not necessarilly
    let n = f3.write(b"abcd").await?;
    println!("Wrote the first {} bytes of 'abcd'.", n);
    //Ok(())

    // write_all()
    let mut f4 = File::create("foo4.txt").await?;
    let ret_f4 = f4.write_all(b"abcdefgh").await?;
    //let ret_f4 = f4.write_all(b"abcdefgh").await;
    dbg!(ret_f4);

    // tokio::io::copy
    let mut reader: &[u8] = b"hello";
    let mut f5 = File::create("f5.txt").await?;
    let ret_copy = io::copy(&mut reader, &mut f5).await?;
    dbg!(ret_copy);

    let mut reader2 = File::open("foo1.txt").await?;
    let mut f6 = File::create("f6.txt").await?;
    let ret_copy = io::copy(&mut reader2, &mut f5).await?;
    dbg!(ret_copy);

    Ok(())
}
