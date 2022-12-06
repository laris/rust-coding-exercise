#![allow(unused)]
#![feature(round_char_boundary)]
fn main() {
//let s = "0123abc❤️🧡💛💚💙💜";
let s = "❤️🧡💛💚💙💜";
//assert_eq!(s.len(), 26);
//assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
println!("{:?}",&s[..closest]);

for c in s.chars() {
    println!("{:?}", c);
}

println!("&str slice len: {}", s.len());
let mut cursor = 0;
let mut current = (0, 0);
let mut next = (0, 0);
let mut char_len_bytes = 0;
loop {
    if cursor == s.len() { break }
    current = (s.floor_char_boundary(cursor), s.ceil_char_boundary(cursor));
    cursor += 1;
    next = (s.floor_char_boundary(cursor), s.ceil_char_boundary(cursor));

    if current != next && current.1 - current.0 == 0  && current.0 != next.0 
        {
            println!("{:2} {:?} {:?} {:?}", cursor, current, next, &s[current.0..=current.1]);
        } else if current != next && current.1 - current.0 == 0  && current.0 == next.0 
        {
        println!("{:2} {:?} {:?} {:?}", cursor, current, next, &s[next.0..next.1]);
        }
}

}
