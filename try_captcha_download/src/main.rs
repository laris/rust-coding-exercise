//use hyper::Client;
//use hyper_tls::HttpsConnector;
//use hyper::body::HttpBody as _;
//use tokio::io::{stdout, AsyncWriteExt as _};
/* 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let uri = "https://u.ky-express.com/diyService/serviceOrderLoc".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
 */

/*
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
 */

//    Ok(())
//}
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
extern crate base64;
extern crate image_base64;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
static FILE_NAME: &'static str = "nyan";
use std::time::Duration;

use reqwest;
use reqwest::header::*;
//use reqwest::Client::*;
//use reqwest::ClientBuilder::*;
//use reqwest::blocking::*;
//use reqwest::header::{HeaderMap, ACCEPT, HOST, USER_AGENT, CONTENT_TYPE};

fn gen_default_headers() -> HeaderMap {
    let mut default_headers = HeaderMap::new();
    //Content-Type: application/json
    default_headers.insert(CONTENT_TYPE, r#"application/json"#.parse().unwrap());
    //Accept: application/json, text/plain, */*
    default_headers.insert(ACCEPT, r#"application/json, text/plain, */*"#.parse().unwrap());
    //Accept-Encoding: gzip, deflate, br
    default_headers.insert("Accept-Encoding", r#"gzip, deflate, br"#.parse().unwrap());
    //Accept-Language: en-US,en;q=0.9
    default_headers.insert("Accept-Language", r#"zh"#.parse().unwrap());
    //Host: u.ky-express.com
    default_headers.insert(HOST, "https://u.ky-express.com/router/rest".parse().unwrap());
    //Origin: https://u.ky-express.com
    //Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.3 Safari/605.1.15
    //User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.3 Safari/605.1.15
    default_headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36".parse().unwrap());
    //Referer: https://u.ky-express.com/diyService/serviceOrderLoc/
    default_headers.insert(REFERER, "https://u.ky-express.com/diyService/serviceOrderLoc/".parse().unwrap());
    //Content-Length: 10
    //Connection: keep-alive
    default_headers.insert("Connection", "keep-alive".parse().unwrap());
    //Cookie: Hm_lpvt_14abfe1895f1102e0c0811ad3bb1dbcc=1644410346; Hm_lvt_14abfe1895f1102e0c0811ad3bb1dbcc=1642482698,1644396009
    //appkey: 80003
    default_headers.insert("appkey", "80003".parse().unwrap());
    //method: gw.verifycode.get
    default_headers.insert("method", "gw.verifycode.get".parse().unwrap());
    
    default_headers
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
//async fn main() {
//    println!("Customized header:\n{:#?}", gen_default_headers());
    download_captcha().await
    //Ok(())
}

async fn download_captcha() -> Result<(), reqwest::Error> {

    let mut map_json_verifycode = std::collections::HashMap::new();
    map_json_verifycode.insert("type", 1);
    println!("Request json data:\n{:#?}", map_json_verifycode);
/*
    let client = reqwest::Client::new();
    let resp = client
        .post("https://u.ky-express.com/diyService/serviceOrderLoc/")
        .headers(gen_default_headers())
        .json(&map_json_verifycode)
        .send()
        .await?
        .json()
        .await?;
    println!("Resp: {:?} {}\n", resp.version(), resp.status());
    println!("Headers: {:#?}\n" , resp.headers());
    Ok(());
*/

    let url = "https://u.ky-express.com/router/rest";

    let client = reqwest::Client::new();
    println!("Client init:\n{:?}", client);
    let reqbuilder_result = client
        .post(url)
        //.header(CONTENT_TYPE, "application/json")
        .header(HOST, "u.ky-express.com")
        .header("method", "gw.verifycode.get")
        .header("appkey", 80003)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "type": 1
        }))
        .build()
        ;
    println!("Req Builder Result:\n{:?}", reqbuilder_result);

    let resp_result = reqwest::Client::execute(&client, reqbuilder_result.unwrap())
        .await
        ;
    println!("Response Result:\n{:#?}", resp_result);

/*
    let resp_text = resp_result.unwrap()
        .text()
        .await
        ;
    println!("text:\n{:#?}", resp_text);
 */

/*
    let resp_json = match resp_result.unwrap().json().await {
        Ok(value) => value,
        Err(e) => {
        error!("获取库存出错:{}", e);
        return Err(format!("{}", e));
        }
    };
    let resp_json = resp_result.unwrap()
        .json()
        .await?
        ;
    println!("resp_json:\n{:#?}", resp_json);
    Ok(())
 */    
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Debug)]
    struct resp_json_code{
        success: bool,
        code: usize,
        msg: String,
        data: resp_json_data,
    }
    #[derive(Deserialize, Debug)]
    struct resp_json_data {
        uuid: String,
        img: String,
    }

    let resp_json = resp_result.unwrap()
        .json::<resp_json_code>()
        .await?
        ;
        //.json(&map_json_verifycode)
       //.send()
        //.await;
        //.text()
        //.await
    println!("resp_json:\n{:#?}", resp_json);

    println!("image base64 orignial code:\n{}", resp_json.data.img);

//    use base64::Base64Mode;

    //let bytes = base64::decode_mode(resp_json.data.img, Base64Mode::Standard).unwrap();
    let bytes = base64::decode(strip_characters(resp_json.data.img.as_str(), "\n")).unwrap();
    println!("{:?}", bytes);
    //println!("{:?}", base64::encode(&bytes));

    // concert base64 to image
    let mut img_base64_repaired_from_malformed_76byte_mime: String = strip_characters(resp_json.data.img.as_str(), "\n");
    println!("{:?}", img_base64_repaired_from_malformed_76byte_mime);
    //println!("{:?}", img_base64_repaired_from_malformed_76byte_mime.insert_str(0,"foo"));

//    let img = image_base64::from_base64(img_base64_repaired_from_malformed_76byte_mime.insert_str(0,"data:image/jpeg;base64,"));
    let img = image_base64::from_base64(img_base64_repaired_from_malformed_76byte_mime);
    println!("{:?}", img);
    //let img_path = "001.jpg";
    //let base64 = image_base64::to_base64(image_path); 
    let mut output = File::create("img-001.jpg").unwrap();
        /* 
        &Path::new(
        &format!(
        "output{}{}.{}",
        MAIN_SEPARATOR, FILE_NAME, "jpg" 
        )))
        .unwrap();
*/
    use std::io::prelude::*;
    //output.write_all(img.as_slice()).unwrap();
    output.write_all(&img).unwrap();
    //output.sync_all();
    Ok(())
    /*
    let client = reqwest::Client::builder()
        //.default_headers(gen_default_headers())
        .build()
        .unwrap();
    println!("ClientBuilder obj:\n{:?}", client);
    let resp = client
        .post(url)
        //.json(&map_json_verifycode)
        .send()
        .await
        .unwrap();
    println!("response :\n{:#?}", resp);
*/

}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}
