#![allow(unused_variables)]
#![allow(unreachable_patterns)]
fn main() {
    let chars = vec!['A', 'B', 'C', 'D'];
    print_match_char(chars);
    let chars = vec!['A'];
    print_match_char(chars);

    // overlapping
    println!("match overlapping");
    let chars = vec![];
    print_match_char_overlapping(chars);
    let chars = vec!['A'];
    print_match_char_overlapping(chars);
    let chars = vec!['A', 'B'];
    print_match_char_overlapping(chars);
    // prevent overlapping
    println!("match overlapping");
    let chars = vec![];
    print_match_char_prevent_overlapping(chars);
    let chars = vec!['A'];
    print_match_char_prevent_overlapping(chars);
    let chars = vec!['A', 'B'];
    print_match_char_prevent_overlapping(chars);
    // guards
    println!("match guards");
    let nums:Vec<u8> = vec![1, 8, 9];
    print_match_num_with_pat_guard(nums);
    let nums:Vec<u8> = vec![8, 9];
    print_match_num_with_pat_guard(nums);
    let nums:Vec<u8> = vec![7, 8, 9];
    print_match_num_with_pat_guard(nums);
    let nums:Vec<u8> = vec![];
    print_match_num_with_pat_guard(nums);
}

fn print_match_char(chars_array: Vec::<char>) {
    match chars_array.as_slice() {
        [first, .., last] => println!("first:{first}, last:{last}"),
        [one, two, ..] => println!("one:{one}, two:{two}"),
        [.., last] => println!("last char:{last}"),
        [single] => println!("single char:{single}"),
        [] => println!("empty char array"),
    }
}

fn print_match_char_overlapping(arr: Vec::<char>) {
    //println!("Vec len: {}", arr.len());
    match arr.as_slice() {
        [] => println!("empty"),            // 0
        [a, ..] => println!("{a}"),          // 1 + 0+
        [a, b, ..] => println!("{a}, {b}"),       // 2 + 0+
        [a, b, c, ..] => println!("{a}, {b}, {c}"),    // 3 + 0+
    }
}
fn print_match_char_prevent_overlapping(arr: Vec::<char>) {
    //println!("Vec len: {}", arr.len());
    match arr.as_slice() {
        [a, b, c, ..] => println!("{a}, {b}, {c}"),    // 3 + 0+
        [a, b, ..] => println!("{a}, {b}"),       // 2 + 0+
        [a, ..] => println!("{a}"),          // 1 + 0+
        [] => println!("empty"),            // 0
    }
}

fn print_match_num_with_pat_guard(arr: Vec<u8>) {
    match arr.as_slice() {
        [first @ 1..=3, rest @ ..] => println!("first always 1, 2, 3: {first:?}, rest is the remaining slice:{rest:?}"),
        [single] if single == &5 || single == &6 => println!("single must 5 or 6: {single:?}"),
        [a, b] => println!("two elements: {a:?}, {b:?}"),
        [a, ..] => println!("one element: {a:?}"),
        [] => println!("empty"),
    }
}
