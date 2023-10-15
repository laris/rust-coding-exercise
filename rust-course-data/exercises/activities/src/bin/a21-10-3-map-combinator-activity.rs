// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not
#![feature(type_name_of_val)]

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let user_name = "sam";
    let user = find_user(user_name).map(|user_id| User { user_id, name: user_name.into() });
    println!("{:?}", user_name);
    println!("type: {:?}", std::any::type_name_of_val(&user));

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("not found"),
    }
    // not found
    let user_name = "jimmy";
    let user = find_user(user_name).map(|user_id| User { user_id, name: user_name.into() });
    println!("{:?}", user_name);
    println!("type: {:?}", std::any::type_name_of_val(&user));

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("not found"),
    }
}
