fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    let v = {
        let mut x = 1;
        x += 2
    };
    assert_eq!(v, ());

}
