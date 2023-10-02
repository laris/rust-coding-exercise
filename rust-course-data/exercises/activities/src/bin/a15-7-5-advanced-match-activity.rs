// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's holder
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage("Tommy".into(), 120.00),
        Ticket::Vip("Jimmy".to_owned(), 300.00),
        Ticket::Standard(50.00),
    ];

    for ticket in tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Ticket::Backstage(holder, price) => println!("Backstage's holder: {}, price: {}", holder, price),
            Ticket::Vip(holder, price) => println!("Vip's holder: {}, price: {}", holder, price),
            Ticket::Standard(price) => println!("Standard's price: {}", price),
        }
    }

}
