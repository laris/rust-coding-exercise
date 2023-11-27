fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100);
    for i in 0..100 { vec.push(i); }
    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);
}
