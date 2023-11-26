use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "stuff".to_owned() });
    lockers.insert(2, Contents { content: "shirt".to_owned() });
    lockers.insert(3, Contents { content: "gym shorts".to_owned() });

    println!("lockers: {:?}", lockers);

    for (locker_number, content) in lockers.iter() {
        println!("number: {:?}, content: {:?}", locker_number, content);
    }
}
