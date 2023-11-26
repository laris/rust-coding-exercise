struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}
fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".into(),
            count: 1,
        },

        LineItem {
            name: "fruit".to_string(),
            count: 3,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("name: {}, count: {}", item.name, item.count);
    }
}
