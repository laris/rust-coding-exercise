fn main() {
    let s1 = String::from("hi, 中国");
    let h  = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[4..7];
    assert_eq!(h1, "中");
}
