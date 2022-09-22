//#![feature(nll)]

use std::collections::HashMap;

fn process_or_default4(map: &mut HashMap<String, String>, key: String)
{
    match map.get_mut(&key) {
        Some(value) => println!("key: {:#?}, value: {:#?}", key, value),
        None => { map.insert(key, String::new()); }
    }
}

fn main() {
    let mut map = HashMap::<String, String>::new();
    let key = String::from("abc");
    let key2 = String::from("abc");
    let key3 = key.clone();
    process_or_default4(&mut map, key);
    process_or_default4(&mut map, key2);
    process_or_default4(&mut map, key3);
}
