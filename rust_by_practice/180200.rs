fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{x}");
    }

    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{x}");
    }

    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() {
        *x += 1;
        println!("{x}");
    }

    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{x}");
    }
}
