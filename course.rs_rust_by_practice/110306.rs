use std::hash::BuildHasherDefault;
use std::collections::HashMap;
//extern crate twox_hash;
use twox_hash::XxHash64;

fn main() {
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
