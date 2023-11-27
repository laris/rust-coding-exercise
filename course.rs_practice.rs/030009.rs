fn main() {
    let (x, y);
    (x,..) = (3,4,5);
    [.., y] = [1, 2, 3];
    assert_eq!([x,y], [3, 3]);
}
