fn main() {
    let mut v = Vec::from([1, 2, 3, 4, 5]);
    for i in 0..5 {
        println!("{:?}", v[i]);
    }
    
    for i in 0..5 {
        v[i] += 1;
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);
    println!("Ok");


    let mut w = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", w.get(i));
    }

    for i in 0..5 {
        if let Some(x) = w.get(i) {
            w[i] = x + 1;
        } else {
            w.push(i+2)
        }
    }

    println!("{w:?}");
}
