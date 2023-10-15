#![allow(dead_code)]

fn main() {
    let maybe_user = Some("Jerry");
    // match all condition
    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user"),
    }

    // same with match all condition
    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user");
    }

    // only choose Some condition or 
    // forcus on one condition
    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    }

    // match enumeration and focus on interesting enum
    #[derive(Debug)]
    enum Color{
        Red,
        Green,
        Blue,
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("its red!");
    } else {
        println!("its not red!");
    }

}
