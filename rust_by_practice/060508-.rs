fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    //let Person { name, ref age } = person;
    let Person { ref name, ref age } = person;
    println!("The person's age is {age}");
    println!("The person's name is {name}");
    println!("The person's struct is {person:?}");
    println!("The person's age from person struct is {}", person.age);
}
