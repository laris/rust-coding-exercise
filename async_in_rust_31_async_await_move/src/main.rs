// async move
// 不同的 async 块可以访问相同的变量s，只要它们都在s的作用域范围内执行
async fn blocks() {
    let s = String::from("Hello World!");
    let future_one = async { println!("{:?}", s); };
    let future_two = async { println!("{:?}", s); };

    futures::future::join(future_one, future_two); // need run in cargo with futures create
}

fn main() {
    println!("Hello, world!");
}
