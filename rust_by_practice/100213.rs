#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T) 
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{

}

fn main() {
    check_size([0u8; 768]);
    check_size([0u8; 767]);
    check_size([0u8; 767]);
    check_size([0u8; 767]);
    check_size([0u8; 767]);
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
}

pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
