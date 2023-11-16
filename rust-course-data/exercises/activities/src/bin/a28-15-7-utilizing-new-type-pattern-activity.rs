// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
use anyhow;
use thiserror::Error;
use std::fmt::Display;

#[derive(Debug, Error)]
enum Color {
    #[error("{self}")]
    Black,
    #[error("{self}")]
    Blue,
    #[error("{self}")]
    Brown,
    #[error("{self}")]
    Custom(String),
    #[error("{self}")]
    Gray,
    #[error("{self}")]
    Green,
    #[error("{self}")]
    Purple,
    #[error("{self}")]
    Red,
    #[error("{self}")]
    White,
    #[error("{self}")]
    Yellow,
}

#[derive(Debug)]
struct ShirtColor(Color);
impl Display for ShirtColor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0);
        Ok(())
    }
}

#[derive(Debug, Error)]
enum ShirtColorErr {
    #[error("this color {} is not allow", .0)]
    NoAllowedColor(Color),
}
/*
impl Display for ShirtColorErr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self);
        Ok(())
    }
}
*/
impl ShirtColor { 
    fn new(color: Color) -> Self { Self(color) }
    fn new_check(color: Color) -> Result<Self, ShirtColorErr> { 
        match color {
            c@Color::Purple => Err(ShirtColorErr::NoAllowedColor(c)),
            c@_ => Ok(Self(c)),
        }
    }
}
#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor { fn new(color: Color) -> Self { Self(color) } }
#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor { fn new(color: Color) -> Self { Self(color) } }

fn print_shirt_color(color: ShirtColor) { println!("Shirt color = {:?}", color); }
fn print_shoes_color(color: ShoesColor) { println!("Shoes color = {:?}", color); }
fn print_pants_color(color: PantsColor) { println!("Pants color = {:?}", color); }

fn main() {
    let shirt_color = ShirtColor::new(Color::Black);

    let shirt_color_purple = ShirtColor::new_check(Color::Purple);
    println!("{shirt_color_purple:?}");

    //let shirt_err_color: anyhow::Error = ShirtColor::new_check(Color::Purple).unwrap_err().into();
    //println!("{shirt_err_color:?}");

    let shirt_ok_color = ShirtColor::new_check(Color::Red);
    println!("{shirt_ok_color:?}");

    let shoes_color = ShoesColor::new(Color::Red);
    let pants_color = PantsColor::new(Color::Blue);
    print_shirt_color(shirt_color);
    print_shoes_color(shoes_color);
    print_pants_color(pants_color);
}
