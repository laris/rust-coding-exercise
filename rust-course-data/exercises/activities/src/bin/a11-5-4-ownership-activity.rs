// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(grocery: &GroceryItem) {
    println!("Quantify of Grocery: {}", grocery.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(grocery: &GroceryItem) {
    println!("Id number of Grocery: {}", grocery.id);
}
fn main() {
    let grocery = GroceryItem {
        quantity: 100,
        id: 10,
    };

    display_quantity(&grocery);
    display_id(&grocery);
}
