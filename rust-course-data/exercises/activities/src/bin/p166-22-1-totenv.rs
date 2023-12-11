//# dotenv = "*"

use std::env;
use dotenv::dotenv;

fn main() {
    //let port = match env::var("PORT") {
    let port = match env::var("PROXY_PORT") {
        Ok(port) => port,
        Err(_) => "555".to_owned(),
    };
    println!("port: {port}");

    dotenv().ok();
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "666".to_owned(),
    };
    println!("port: {port}");
}