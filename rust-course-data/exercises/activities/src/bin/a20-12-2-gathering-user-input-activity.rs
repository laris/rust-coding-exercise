// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)


// * Use an enum to store the possible power states
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    //UnknownState,
}
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
       //let state = state.trim().to_lowercase().as_str();
       match state.trim().to_lowercase().as_str() {
           "off"       => Some(PowerState::Off),
           "sleep"     => Some(PowerState::Sleep),
           "reboot"    => Some(PowerState::Reboot),
           "shutdown"  => Some(PowerState::Shutdown),
           "hibernate" => Some(PowerState::Hibernate),
           _           => None,

       }
    }

    fn print_power_action(&self) {
        use PowerState::*;
        match self {
            Off       => println!("system is power off"),
            Sleep     => println!("system is sleep"),
            Reboot    => println!("system will reboot"),
            Shutdown  => println!("system will shutdown"),
            Hibernate => println!("system will hibernate"),
        }
    }
}


/*
fn print_msg(power: PowerState) {
    use PowerState::*;
    match power {
        Sleep     => println!("system is sleep"),
        Off       => println!("system is power off"),
        Reboot    => println!("system will reboot"),
        Shutdown  => println!("system will shutdown"),
        Hibernate => println!("system will hibernate"),
        UnknownState => println!("input error"),
    }
}
*/
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

fn read_input() -> std::io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
    /*
    match buffer.trim().to_lowercase().as_str() {
        "off"       => Ok(Some(PowerState::Off)),
        "sleep"     => Ok(Some(PowerState::Sleep)),
        "reboot"    => Ok(Some(PowerState::Reboot)),
        "shutdown"  => Ok(Some(PowerState::Shutdown)),
        "hibernate" => Ok(Some(PowerState::Hibernate)),
        _           => Ok(None),
    }
    */
}

fn main() {
    let state = PowerState::new(read_input().unwrap().as_str());
    match state {
        Some(powerstate) => powerstate.print_power_action(),
        None => println!("input is not valid keyword of PowerState"),
    }
}
