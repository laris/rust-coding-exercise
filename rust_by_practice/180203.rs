pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v = vec![1, 2];
    //let mut v = IntoIterator::into_iter(v);
    let mut v = v.into_iter();
    assert_eq!(v.next(), Some(1));
    assert_eq!(v.next(), Some(2));
    assert_eq!(v.next(), None);
}
