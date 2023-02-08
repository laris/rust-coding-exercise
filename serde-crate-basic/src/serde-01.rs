// [serde json 常用实践](https://betheme.net/news/txtlist_i113997v.html?action=onClick)

// 将json字符串转换为json对象
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some json input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    "#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("{v:#?}");
    Ok(())
}

use serde::{Serialize, Deserialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


// 将json字符串转换为强类型对象
// 将json对象转换成结构体，需要在结构体上写#[derive(Serialize, Deserialize)]
//use serde::{Deserialize, Serialize};
//use serde_json::Result;
fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
    "#;

// Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;
    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    println!("{p:#?}");

    Ok(())
}

fn main() {
    untyped_example();
    typed_example();
// 将自然json转换为json对象
// 这里说的自然json不是字符串，就是一个自然json表达式
    use serde_json::json;
    // The type of `john` is `serde_json::Value`
    let john = json!(
      {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
      }
    );

    println!("first phone number: {}", john["phones"][0]);
    println!("{john}");

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
// ----  加入表达式
    let full_name = "John Doe";
    let age_last_year = 42;
    fn random_phone() -> u32 { 573821431 }
    // The type of `john` is `serde_json::Value`
    let john = json!(
    {
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
    });
    println!("{john}");
// ---- 将结构体转换为json字符串
//use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

/*
字符串的默认值
在反序列化时可能遇到字段不存在，或者字段可选，有时存在有时不存在的情况。我们可以指定字段的默认值，如果反序列化时不存在该字段的值，那么就使用默认值。
#[serde(default)]
如果反序列化时不存在该值，会使用Default::default()。这里使用的时rust默认值。
#[serde(default = “path”)]
如果反序列化时不存在该值，会使用一个函数以获取默认值。在“path”中指定函数路径，这里使用的是自定义的默认值。

当使用#[serde(default = “path”)]时，函数可能定义在其他文件中。可以这样使用：
#[serde(default="super::common::default::get_default")]
*/

//use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Request {
    // Use the result of a function as the default if "resource" is
    // not included in the input.
    #[serde(default = "default_resource")]
    resource: String,

    // Use the type's implementation of std::default::Default if
    // "timeout" is not included in the input.
    #[serde(default)]
    timeout: Timeout,

    // Use a method from the type as the default if "priority" is not
    // included in the input. This may also be a trait method.
    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

fn default_resource() -> String {
    "/".to_string()
}

/// Timeout in seconds.
#[derive(Deserialize, Debug)]
struct Timeout(u32);
impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

#[derive(Deserialize, Debug)]
enum Priority { ExtraHigh, High, Normal, Low, ExtraLow }
impl Priority {
    fn lowest() -> Self { Priority::ExtraLow }
}

fn main() {
    let json = r#"
        [
          {
            "resource": "/users"
          },
          {
            "timeout": 5,
            "priority": "High"
          }
        ]
    "#;

    let requests: Vec<Request> = serde_json::from_str(json).unwrap();

    // The first request has resource="/users", timeout=30, priority=ExtraLow
    println!("{:?}", requests[0]);

    // The second request has resource="/", timeout=5, priority=High
    println!("{:?}", requests[1]);
}


}
