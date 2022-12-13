fn main() {
/*
fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where Self: Sized, F: FnMut(&Self::Item) -> B;

fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where Self: Sized, F: FnMut(&Self::Item) -> B;
*/

    use std::collections::HashMap;
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449); 
    populations.insert("Greenhorn", 2); 
    populations.insert("Boring", 7_762); 
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop), Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop), Some((&"Greenhorn", &2)));
    assert_eq!(populations.into_iter().min_by_key(|&(_name, pop)| pop), Some(("Greenhorn", 2)));
}
