#![allow(dead_code)]
#![allow(unused_variables)]

use std::future::Future;

// async/await 的使用
// `foo()` 返回一个实现了 `Future<Output = u8>` 的类型。
// `foo().await` 将会产生一个 u8 类型的值。
async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
    // 这个 `async` 块会产生一个实现了 `Future<Output = u8>` 的类型。
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

// async lifetime
// 与普通的函数不一样，async fn 会获取引用或其他非静态生命周期的参数，然后返回被这些参数的生命周期约束的 Future：
async fn foo_lifetime(x: &u8) -> u8 { *x }

// 这与上面的函数完全等价
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
// 把带有引用参数的 async fn 转化为静态 Future 的解决方法是：把参数和对 async fn 的调用封装到 async 块中
async fn borrow_x(x: &u8) -> u8 { *x }

fn bad() -> impl Future<Output = u8> {
    let x = 5;
    // borrow_x(&x) // ERROR: `x` does not live long enough
    // borrowed value does not live long enough
    // argument requires that `x` is borrowed for `'static`
    async move { x }
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

// async move
// this part need executor, so need run with cargo and import futures crate
// 不同的 async 块可以访问相同的变量s，只要它们都在s的作用域范围内执行
async fn blocks() {
    let s = String::from("Hello World!");
    let future_one = async { println!("{:?}", s); };
    let future_two = async { println!("{:?}", s); };

    // futures::future::join(future_one, future_two); // need run in cargo with futures crate
}

fn main() {

}