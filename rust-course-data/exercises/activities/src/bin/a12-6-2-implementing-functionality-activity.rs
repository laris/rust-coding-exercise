// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        println!("box's width: {:?}, height: {:?}, depth: {:?}", self.width, self.height, self.depth);
    }
}
// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    Brown,
    Red,
    Blue,
    Green,
}
impl Color {
    fn print(&self) {
        /*
        match self {
            Color::Brown => println!("brown"),
            Color::Red=> println!("red"),
            _ => println!("other color"),        
        }
        */
        println!("box's color: {:?}", self);
    }
}
impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
        weight,
        color,
        dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("box's weight: {:?}", self.weight);
    }
}
fn main() {
    let my_box = ShippingBox::new(10.00, Color::Red, Dimensions { width: 20.00, depth: 30.0, height: 100.0});
    my_box.print();
}
