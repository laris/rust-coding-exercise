#![allow(unused_must_use)]

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        //other @ _ => Err(format!("menu choice {{'{}'}} not found", other)),
        //other @ _ => Err(format!("menu choice {{\"{}\"}} not found", other)),
        other @ _ => Err(format!(r#"menu choice {{"{}"}} not found"#, other)),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}
fn main() {
    // MainMenu
    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    println!("choice = {:?}", choice);
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }
    // Err
    let choice: Result<MenuChoice, _> = get_choice("leave");
    println!("choice = {:?}", choice);
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }
    // Start
    pick_choice("start");
    // end
    let choice = pick_choice("end");
    println!("choice value = {:?}", choice);
}
