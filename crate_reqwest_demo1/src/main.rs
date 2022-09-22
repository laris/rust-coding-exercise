/*
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
*/

/*
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://hyper.rs").await?;
    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

// The [cfg(not(target_arch = "wasm32"))] 
// above prevent building the tokio::main function
// for wasm32 target, because tokio isn't compatible with wasm32.
// If you aren't building for wasm32, you don't need that line.
// The two lines below avoid the "'main' function not found" error when building for wasm32 target.
#[cfg(target_arch = "wasm32")]
fn main() {}
 */

extern crate env_logger;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Get https://www.rust-lang.org");

    let mut res = reqwest::blocking::get("https://www.rust-lang.org/")?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;
    println!("\n\nDone.");
    Ok(())
}

