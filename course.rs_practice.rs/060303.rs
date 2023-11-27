fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..=3];
    assert_eq!(slice, &[2,3,4]);
}
