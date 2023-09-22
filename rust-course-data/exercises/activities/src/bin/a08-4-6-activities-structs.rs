// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
#[derive(Debug)]
enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sparkling => println!("drink flavor: sparkling"),
        Flavor::Sweet     => println!("drink flavor: sweet"),
        Flavor::Fruity    => println!("drink flavor: fruity"),
        _ => println!("Error"),
    }
}
fn main() {
    let sweet_drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 1.0
    };
    print_drink(sweet_drink);

    let fruity_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 2.0
    };
    print_drink(fruity_drink);
}
