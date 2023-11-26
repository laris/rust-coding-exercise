#![feature(type_name_of_val)]

fn main() {
    //---- if else expression
    let my_num = 3;
    let is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };
    // test std::any::type_name_of_val() to get expression return type
    let my_num = 42;
    let type_name = std::any::type_name_of_val(&my_num);
    println!("Type of `my_num` is: {}", type_name);
    // confirm return type
    println!("Type of `is_lt_5` is: {}", std::any::type_name_of_val(&is_lt_5));
    // direct assign compare result as expression to value
    let is_lt_5 = my_num < 5;
    println!("Type of `is_lt_5` is: {}", std::any::type_name_of_val(&is_lt_5));
    // match expression
    let my_num = 3;
    let message = match my_num {
        1 => "Hello",
        _ => "goodbye",
    };
    println!("Type of `message` is: {}", std::any::type_name_of_val(&message));
    // another demo
    enum Menu {
        Burger,
        Fries,
        Drink,
    }

    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" { true } else { false }
        },
        _ => true,
    };
    println!("Type of `order_placed` is: {}", std::any::type_name_of_val(&message));
    // refine upper demo 
    #[derive(Debug, Eq, PartialEq)]
    enum DrinkType {
        water,
        beer,
        juice,
    }
    enum OrderMenu {
        Burger,
        Fries,
        Drink(DrinkType),
    }

    let paid = true;
    let item = OrderMenu::Drink(DrinkType::water);
    //let drink_type = "water";
    let order_placed = match item {
        OrderMenu::Drink(drink_type) => {
            if drink_type == DrinkType::water { true } else { false }
        },
        _ => true,
    };
    println!("Type of `order_placed` is: {}", std::any::type_name_of_val(&message));
}
