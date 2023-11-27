use std::num::ParseIntError;

type Res<T> = Result<T, ParseIntError>;

fn multiply(n1str: &str, n2str: &str) -> Res<i32> {
    n1str.parse::<i32>().and_then(
        |n1| {
            n2str.parse::<i32>().map(
                |n2| 
                    n1 * n2
            )
        }
    )
}

// 同样, 这里也使用了类型别名来简化代码
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!")
}
