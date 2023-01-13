fn main() {
    let t = (String::from("hello"), String::from("world"));

    let (ref s1, ref s2) = t;
    println!("{s1:?}, {s2:?}, {t:?}");

    let (s1, s2) = t.clone();
    println!("{s1:?}, {s2:?}, {t:?}");
}
