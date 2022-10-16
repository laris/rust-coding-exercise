#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do some async work and return
        "return value"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("Got output: {}", out);
}
