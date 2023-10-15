#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(type_name_of_val)]
fn maybe_num() -> Option<i32> {None}
fn maybe_word() -> Option<String> {None}

fn main() {
    let plus_one = match maybe_num() {
        Some(num) => Some(num + 1),
        None => None,
    };

    let plus_one = maybe_num().map(|num| num + 1);

    // convert String to usize
    let word_length = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);
    println!("{:?}", std::any::type_name_of_val(&word_length));
}
