fn main() {
    /*
    fn fold<A, F>(self, init: A, f: F) -> A
        where Self: Sized, F: FnMut(A, Self::Item) -> A;
    */

    let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];

    // See also: the `join` method on slices, which won't 
    // give you that extra space at the end.
    let pangram = a.iter()
        .fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");
    assert_eq!(a.join(" "), "Pack my box with five dozen liquor jugs");
    
    let weird_pangram = a.iter()
        .rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");
    println!("{:?}", a.iter().rev().cloned().collect::<Vec<_>>());
    println!("{:?}", a.iter().rev().cloned().collect::<Vec<_>>().as_slice().join(" "));
}
