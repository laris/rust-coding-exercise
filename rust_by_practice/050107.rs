fn main() {
    let x = Box::new(5);
    //let ref mut y = 0;
    let mut y = Box::new(0);
    *y = 4;
    assert_eq!(*x, 5);
}
