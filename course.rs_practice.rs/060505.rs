struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = build_person("Name".to_string(), 8);
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}
