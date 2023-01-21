fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..];
    //slice3.push(4);

    //assert_eq!(slice3, &[1, 2, 3]);
    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Ok");
}
