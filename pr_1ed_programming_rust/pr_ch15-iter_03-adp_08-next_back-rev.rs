
/*
trait DoubleEndedIterator: Iterator {
    fn next_back(&mut self) -> Option<Self::Item>;
}
*/

fn main() {
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(),      Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(),      Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(),      None);

    /*
    fn rev(sel) -> impl Iterator<Item=Self>
        where Self: Sized + DoubleEndedIterator;
    */

    let meals = ["breakfast", "lunch", "dinner"];

    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);
}
