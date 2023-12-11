//# derive_more ="*"

use derive_more::Display;
#[derive(Display)]
#[display(fmt = "Item: {}, Quantity: {}", item, qty)]
struct Order {
    item: String,
    qty: usize,
}

#[derive(Display)]
#[display(fmt = "Grocery item: {}")]
enum GroceryItem {
    #[display(fmt = "Bread slices: {}", _0)]
    Bread(usize),
    #[display(fmt = "Fruit")]
    Fruit,
    #[display(fmt = "Ounces of meat: {}", _0)]
    Meat(usize),
}
fn main() {
    let order = Order { item: "Berg".into(), qty: 1, };
    println!("{order}");

    let bread = GroceryItem::Bread(10);
    let fruit = GroceryItem::Fruit;
    let meat  = GroceryItem::Meat(6);
    println!("{bread}");
    println!("{fruit}");
    println!("{meat}");

    // from
    use derive_more::From;
    #[derive(From)]
    struct UserId(i32);
    let user_id: UserId = 15.into();
    let user_id = UserId::from(15);

    #[derive(From)]
    struct Coordinate {
        x: i32,
        y: i32,
    }
    let coord: Coordinate = (3, 2).into(); // tuple
    let coord = Coordinate::from((3, 2));  // tuple

    #[derive(From)]
    enum Material {
        Flooring(usize, usize),
        Wood(usize),
    }
    let floor: Material = (5, 5).into();
    let floor = Material::from((5,5));
    let wood: Material = 10.into();
    let wood = Material::from(10);

    use derive_more::IntoIterator;
    #[derive(IntoIterator)]
    struct Passengers {
        #[into_iterator(owned, ref, ref_mut)]
        names: Vec<String>,
    }

    let mut passengers = Passengers { names: vec![] };
    passengers.names.push("Jimmy".into());
    passengers.names.push("Bob".into());
    passengers.names.push("Tom".into());

    for passenger in &passengers {
        println!("{passenger}");
    }
    for passenger in passengers {
        println!("{passenger}");
    }

    //use derive_more::{Add, From, Sub};
    use derive_more::{Add, Sub};
    #[derive(Debug, From, Add, Sub)]
    struct Point {
        x: i32,
        y: i32,
    }
    let a: Point = (1, 1).into();
    let b: Point = (2, 3).into();
    println!("{:?}", a + b);
}
