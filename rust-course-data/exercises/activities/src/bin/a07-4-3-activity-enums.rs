// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color_name(my_color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match my_color {
        Color::Red => println!("Color: Red"),
        Color::Green => println!("Color: Green"),
        Color::Blue => println!("Color: Blue"),
        _  => println!("Color: other color"),
    }
}

fn main() {
    print_color_name(Color::Green);

    let my_color = Color::Blue;
    print_color_name(my_color);
}
