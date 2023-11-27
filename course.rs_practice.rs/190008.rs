fn main() {
    //let s: str = "hello there!";
    let s: &str = "hello there!";
    //let arr: [u8] = [1, 2, 3];
    let arr: &[u8] = &[1, 2, 3];
    let a = arr[0];
    let ref arr2: [u8;3] = [1, 2, 3];
    let b = arr2[0];

    assert_eq!(a, b);
}
