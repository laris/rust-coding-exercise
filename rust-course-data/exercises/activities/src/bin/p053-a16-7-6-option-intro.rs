#[allow(unused_variables)]
#[allow(dead_code)]
/*
enum Option<T> {
    Some<T>,
    None
}
*/

fn main() {
    struct Customer {
        age: Option<i32>,
        email: String,
    }

    let mark = Customer { age: Some(22), email: "mark@example.com".into(), };
    let becky = Customer { age: None, email: "becky@example.com".into(), };

    match becky.age {
        Some(age) => println!("Customer is {:?} years old", age),
        None => println!("Customer age not provided"),
    }

    struct GroceryItem {
        name: String,
        qty: i32,
    }

    fn find_quantity(name: &str) -> Option<i32> {
        let groceries = vec![
            GroceryItem { name: "bananas".into(), qty: 4, },
            GroceryItem { name: "eggs".into(), qty: 12, },
            GroceryItem { name: "bread".into(), qty: 1, },
        ];
        for item in groceries {
            if item.name == name { return Some(item.qty); }
        }
        None
    }
}
