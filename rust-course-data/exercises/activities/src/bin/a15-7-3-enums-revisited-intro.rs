#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

#[derive(Debug)]
enum PromoDiscount {
    NewUser(User),
    Holidy(String),
}

#[derive(Debug)]
enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
   let mouse_action1 = Mouse::LeftClick; 
   let mouse_action2 = Mouse::Scroll(1); 
   let mouse_action3 = Mouse::Move(0, 0); 

   let promo_discount1 = PromoDiscount::NewUser(User { name: "UserName".into(), age: 30,});
   let promo_discount2 = PromoDiscount::Holidy("New Year".into());
   let discount1 = Discount::Percent(0.8);
   let discount2 = Discount::Flat(2);
   let discount3 = Discount::Promo(promo_discount2);
   let discount4 = Discount::Custom("Special annerversity".into());
}
