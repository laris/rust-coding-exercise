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
use std::collections::HashMap;
/*
#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Self { name, amount }
    }
    fn interactive_get_bill_from_input() -> Result<Bill, Box<dyn std::error::Error>> {
        println!("Please input bill's name:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let name: String = input.trim().into();
        input.clear();
        println!("Please input bill's amount:");
        std::io::stdin().read_line(&mut input)?;
        let amount: f64 = input.trim().parse()?;
        Ok(Self::new(name, amount))
    }
}
*/ 

fn view_bills(bills: &HashMap<String, f64>) {
    for (idx, bill) in bills.iter().enumerate() {
        println!("{idx}: name: {:?} amount: {:?}", bill.0, bill.1);
    }
}

fn interactive_get_bill_from_input() -> Result<(String, f64), Box<dyn std::error::Error>> {
    println!("Please input bill's name:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let name: String = input.trim().into();
    input.clear();
    println!("Please input bill's amount:");
    std::io::stdin().read_line(&mut input)?;
    let amount: f64 = input.trim().parse()?;
    Ok((name, amount))
}

fn add_bills(bills: &mut HashMap<String, f64>) {
    let (name, amount) = interactive_get_bill_from_input().unwrap();
    bills.insert(name, amount);
}

fn remove_bill(bills: &mut HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
    view_bills(bills);
    println!("Which bill do you want to remove? Input name:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    println!("Input:{:?}, Remove entry:{:?}", input.trim(), bills.remove_entry(&input.trim().to_string()));
    view_bills(bills);
    Ok(())
}

//fn edit_bill(bills: &mut Vec<Bill>) -> Result<(), Box<dyn std::error::Error>>{
fn edit_bill(bills: &mut HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>>{
    view_bills(bills);
    println!("Which bill do you want to edit? Input name:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?; // read new input as String
    //let idx: usize = input.trim().parse()?; // parse String to usize
    let new_bill = interactive_get_bill_from_input().unwrap(); // get new bill
    let old_bill = bills.remove_entry(&input.trim().to_string()).unwrap();
    println!("Are you confirm to replace \nthe old bill {old_bill:?}\nwith new bill {new_bill:?}?\nYy/No?");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let confirm = input.trim().to_lowercase();
    match confirm.as_str() {
        "y" | "yes" | "true" => { bills.insert(new_bill.0, new_bill.1); view_bills(bills); },
        "n" | "no" | "false" => { bills.insert(old_bill.0, old_bill.1); view_bills(bills); },
        _ => (),
    }
    Ok(())
}

#[derive(Debug)]
enum Cmd {
    View,
    Add,
    Remove,
    Edit,
    Quit,
    InvalidCmd(String),
}

impl From<&str> for Cmd {
    fn from(cmd: &str) -> Self {
        match cmd {
            "1" | "view"   => Cmd::View,
            "2" | "add"    => Cmd::Add,
            "3" | "remove" => Cmd::Remove,
            "4" | "edit"   => Cmd::Edit,
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
    println!("----------Menu----------");
    println!("1. View bills");
    println!("2. Add bills");
    println!("3. Remove bills");
    println!("4. Edit bills");
    println!("0. Quit");
    println!("Please input the command:");
}

fn main() {
    //let mut bills = vec![Bill { name: "Coffee".into(), amount: 9.99 }];
    let mut bills = HashMap::<String, f64>::new();
    bills.insert("Coffe".into(), 8.88);
    loop {
        print_menu();
        match parse_input() {
            Err(e) => { println!("Invalid cmd: {e:?}, please retry"); continue; },
            Ok(cmd) => {
                match cmd {
                    Cmd::View   => view_bills(&bills),
                    Cmd::Add    => add_bills(&mut bills),
                    Cmd::Remove => remove_bill(&mut bills).expect("todo for Ok(())"),
                    Cmd::Edit   => edit_bill(&mut bills).expect("todo for Ok(())"),
                    Cmd::Quit   => break,
                    e@_ => { println!("Invalid cmd: {e:?}, please retry"); continue; }
                }
            }
        }
    }
}
