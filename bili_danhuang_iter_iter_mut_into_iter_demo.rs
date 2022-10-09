fn main() {
    let mut v1: Vec<u8> = vec![1, 2, 3];
    let v1_iter_mut = v1.iter_mut();
    let mut v2: Vec<&mut u8> = v1_iter_mut.collect();
    *v2[0] = 0;
    println!("{:?}", v1);
    // println!("{:?}", v2);
    //
    let mut v1: Vec<u8> = vec![1, 2, 3];
    let v1_into_iter = v1.into_iter();
    let mut v2: Vec<u8> = v1_into_iter.collect();
    v2[0] = 3;
    // println!("{:?}", v1);
    println!("{:?}", v2);

}