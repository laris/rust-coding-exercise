// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
#[derive(Debug)]
struct Customer {
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
fn try_purchase(customer: &Customer) -> Result<(), String> {
    // * Restricted purchases require that the age of the customer
    //   is at least 21
    /*
    if customer.age < 21 {
        Err(format!("customer's age: {} is < 21 years old", customer.age))
    } else { Ok(()) }
    */
    /*
    match customer.age {
        age if age >= 21 => Ok(()),
        age @ _ => Err(format!("customer's age: {} is < 21", age)),
    }
    */
    /*
    */
    match customer.age {
        age if age < 21 => Err(format!("customer's age: {} is < 21", age)),
        _ => Ok(()),
    }
}

fn main() {
    let ashley = Customer { age: 21 };
    println!("{:?}", try_purchase(&ashley));

    let customer2 = Customer { age: 20 };
    println!("{:?}", try_purchase(&customer2));
}
