fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        panic!("drinked, duang.....peng!")
    }
    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");
    println!("Exercise Failed if printing out this line!");
}
