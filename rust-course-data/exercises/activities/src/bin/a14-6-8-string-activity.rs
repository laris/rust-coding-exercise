// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
#[derive(Debug)]
struct Person {
    // * The color and name should be stored as a String
    age: i32,
    name: String,
    fav_color: String,
}

fn print_data(data: &str) {
    print!("{:?}", data);
}
impl Person {
    fn new(age: i32, name: &str, fav_color: &str) -> Self {
        Self {
            age,
            name: name.into(),
            fav_color: fav_color.into(),
        }
    }
    fn print_name_color(self) {
        println!("name: {} love color: {}", self.name, self.fav_color);
    }
}
fn main() {
    // * Create and store at least 3 people in a vector
    let peoples = vec![
        Person::new(10, "Jayson", "Red"),
        Person::new(9, "George", "Green"),
        Person::new(30, "Anna", "Purple"),
        Person::new(14, "Katie", "Pink"),
    ];
    // * Iterate through the vector using a for..in loop
    for people in peoples {
        // * Use an if expression to determine which person's info should be printed
        // * The name and colors should be printed using a function
        //println!("{:?}", people);
        if people.age <=10 {
            //people.print_name_color();
            print!("print age <= 10: name: ");
            print_data(&people.name);
            print!(", favorate color: ");
            print_data(&people.fav_color);
            println!("");
        }
    }
}
