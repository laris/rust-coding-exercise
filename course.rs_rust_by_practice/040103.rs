fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    assert_eq!("i32".to_string(), type_of(&10));
    assert_eq!("bool".to_string(), type_of(&true));
    println!("{}", type_of(&(0, 3.1)));
    println!("{}", type_of(&[0, 3, 1]));
    println!("{}", type_of(&"better"));
    println!("{}", type_of(&"better".to_string()));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
