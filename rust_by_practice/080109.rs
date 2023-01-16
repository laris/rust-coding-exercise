fn main() {
    let age = Some(30);

    if let Some(age) = age {
        //assert_eq!(age, Some(30));
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is new, value: {age}"),
        _ => ()
    }
}
