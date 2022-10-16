// #[tokio::main]
// async fn main() {
//     println!("hello");
// }

fn main() {
    let mut _rt = tokio::runtime::Runtime::new().unwrap();
    _rt.block_on(async {
        println!("hello");
    })
}