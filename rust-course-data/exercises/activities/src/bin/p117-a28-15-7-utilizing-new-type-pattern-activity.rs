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
    #[error("<enum Color, Error: {self:?}>")]
    Black,
    #[error("<enum Color, Error: {self:?}>")]
    Blue,
    #[error("<enum Color, Error: {self:?}>")]
    Brown,
    #[error("<enum Color, Error: {self:?}>")]
    Custom(String),
    #[error("<enum Color, Error: {self:?}>")]
    Gray,
    #[error("<enum Color, Error: {self:?}>")]
    Green,
    #[error("<enum Color, Error: {self:?}>")]
    Purple,
    #[error("<enum Color, Error: {self:?}>")]
    Red,
    #[error("<enum Color, Error: {self:?}>")]
    White,
    #[error("<enum Color, Error: {self:?}>")]
    Yellow,
}

#[derive(Debug)]
struct ShirtColor(Color);
impl Display for ShirtColor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        //self.0.fmt(fmt);
        //write!(fmt, "{}", self.0);
        write!(fmt, "<Display for struct ShirtColor: {:?}>", self.0);
        Ok(())
    }
}

#[derive(Debug, Error)]
enum ShirtColorErr {
    #[error("<Error for Enum ShirtColorErr: This color {} is not allow>", .0)]
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

fn print_shirt_color(color: ShirtColor) { println!("Shirt color = Display: {}, Debug: {:?}", color, color); }
fn print_shoes_color(color: ShoesColor) { println!("Shoes color = Debug: {:?}", color); }
fn print_pants_color(color: PantsColor) { println!("Pants color = Debug: {:?}", color); }

fn main() {
    let shirt_color = ShirtColor::new(Color::Black);

    let shirt_color_purple = ShirtColor::new_check(Color::Purple);
    println!("Debug: {shirt_color_purple:?}");

    let shirt_err_color: anyhow::Error = ShirtColor::new_check(Color::Purple).unwrap_err().into();
    println!("Debug: {shirt_err_color:?}");
    println!("Display: {shirt_err_color}");

    let shirt_ok_color = ShirtColor::new_check(Color::Red);
    println!("Debug: {shirt_ok_color:?}");
    //println!("{shirt_ok_color}");

    let shoes_color = ShoesColor::new(Color::Red);
    let pants_color = PantsColor::new(Color::Blue);
    print_shirt_color(shirt_color);
    print_shoes_color(shoes_color);
    print_pants_color(pants_color);
}
