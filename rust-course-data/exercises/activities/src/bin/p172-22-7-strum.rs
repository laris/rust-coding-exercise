//# strum = { version = "*", features = ["derive"]}
//# strum_macros = "*"
use std::str::FromStr;
use strum::{EnumCount, EnumIter, IntoEnumIterator, EnumMessage};
use strum_macros::EnumString;
#[derive(Debug, EnumCount, EnumIter)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, EnumMessage, EnumString)]
enum Status {
    #[strum(
        message = "Idle",
        detailed_message = "Waiting for jobs",
        serialize = "i", serialize = "Idle"
        )]
    Idle,
    #[strum(serialize = "p")]
    Processing,
}
fn main() {
    println!("Variant count: {}", Color::COUNT);
    for variant in Color::iter() {
        println!("{:?}", variant);
    }

    let idle = Status::Idle;
    println!("{:?}", idle.get_message());
    println!("{:?}", idle.get_detailed_message());
    let idle = Status::Processing;
    println!("{:?}", idle.get_message());

    let idle = Status::from_str("i"); // Ok(Idle)
    println!("{idle:?}");
    let idle = Status::from_str("Idle"); // Ok(Idle)
    println!("{idle:?}");
    let processing = Status::from_str("p"); // Ok(Processing)
    println!("{processing:?}");
    let processing = Status::from_str("Processing"); // Err(NotFound)
    println!("{processing:?}");
}
