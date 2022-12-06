#![allow(unused)]
fn main() {
let s = "Löwe 老虎 Léopard";
println!("&str slice len: {}\nidx: bytes: bd?: char", s.len());
for (i,c) in s.bytes().enumerate() {
    //let ch_bytes = c.as_bytes();
    println!("{i:2}: {c:4}: {:5}: {:?}",
        s.is_char_boundary(i), 
        char::from_u32(c as u32).unwrap());
}
assert!(s.is_char_boundary(0));
assert!(s.is_char_boundary(1));
assert!(!s.is_char_boundary(2));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));
assert!(s.is_char_boundary(s.len()-1));
// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
}
