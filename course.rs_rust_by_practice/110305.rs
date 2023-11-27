use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    
    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    map.shrink_to_fit();
    assert!(map.capacity() >= 2);

    println!("Ok");

    //----------
    // use std::collections::HashMap;
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to HashMap: {}", v1);

    let v2 = "hello";
    let mut m2 = HashMap::new();
    m2.insert(v2, v1);
    assert_eq!(v2, "hello");

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    m2.insert(v2.clone(), v1);
    assert_eq!(v2, "hello");

    println!("Ok");
}
