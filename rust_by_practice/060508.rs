#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let ref _name = f.name;
    println!("{}, {}, {:?}", f.name, f.data, f);
}
