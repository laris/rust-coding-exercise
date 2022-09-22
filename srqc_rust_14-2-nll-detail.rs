fn main1() {
    let mut v = vec![1,2,3,4,5];
    v.push(v.len());
    println!("{:?}", v);
}


fn main2() {
    let mut data = 100_i32;
    let mut p = &data;
    println!("{}", p);

    data = 101;
    p = &data;
    println!("{}", p);
}


fn main() {
    main1();
    main2();
}
