//# serde = { version = "1.0", features = ["derive"]}
//# serde_json = "*"

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Form {
    email: String,
    name: String,
    age: usize,
}

fn main() {
    let form = Form {
        email: "sample@example.com".to_string(),
        name: "Sample".into(),
        age: 25,
    };
    let serialized = serde_json::to_string(&form).expect("failed to serialize");
    println!("{serialized}");

    let deserialized: Result<Form, _>;
    deserialized = serde_json::from_str(&serialized);
    println!("{deserialized:?}");
}
