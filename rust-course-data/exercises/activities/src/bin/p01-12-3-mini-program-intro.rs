// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

fn view_bills(bills: &Vec<Bill>) {
    for (idx, bill) in bills.iter().enumerate() {
        println!("{idx}: name: {} amount: {}", bill.name, bill.amount);
    }
}

#[derive(Debug)]
enum Cmd {
    View,
    Add,
    Quit,
    InvalidCmd(String),
}
impl From<&str> for Cmd {
    fn from(cmd: &str) -> Self {
        match cmd {
            "1" | "view" => Cmd::View,
            "2" | "add" => Cmd::Add,
            "0" | "q" | "quit" | "exit" => Cmd::Quit,
            cmd@_ => Cmd::InvalidCmd(cmd.into()),
        }
    }
}
fn parse_input() -> std::io::Result<Cmd> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase().as_str().into())
}

fn print_menu() {
    println!("Menu:");
    println!("1. View bills");
    println!("2. Add bills");
    println!("0. Quit");
    println!("Please input the command:");
}
fn main() {
    let mut bills = Vec::<Bill>::new();
    loop {
        print_menu();
        match parse_input() {
            Err(e) => { println!("Invalid cmd: {e:?}, please retry"); continue; },
            Ok(cmd) => {
                match cmd {
                    Cmd::View => view_bills(&bills),
                    Cmd::Quit => break,
                    e@_ => { println!("Invalid cmd: {e:?}, please retry"); continue; }
                }
            }
        }
    }
}
