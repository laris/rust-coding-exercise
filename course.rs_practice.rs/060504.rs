struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("Sunface"),
        age,
    };

    p.age = 30;
    p.name = String::from("Sunfei");
}
