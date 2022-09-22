// disable #![warn(dead_code)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

fn main() {
    //ch01concat::by_cloning();
    //ch01concat::by_moving();
    //ch01concat::by_mutating();
    //ch01format::ch01format();
    //ch01default::ch01default();
    //ch01constructor::ch01constructor();
    //ch01constructor::ch01constructor_cow();
    //ch01builder::ch01builder();
    //ch01parallelism::ch01parallelism();
    //ch01random_number::ch01random_number();

}

// 
pub mod ch01concat {
    pub fn by_moving() {
        let hello = "hello ".to_string();
        let world = "world!";
        let hello_world = hello + world;
        println!("{}", hello_world);
    }
    pub fn by_cloning() {
        let hello = "hello ".to_string();
        let world = "world!";
        let hello_world = hello.clone() + world;
        println!("{}", hello_world);
    }
    pub fn by_mutating() {
        let mut hello = "hello ".to_string();
        let world = "world!";
        hello.push_str(world);
        println!("{}", hello);
    }
}

pub mod ch01format {
    pub fn ch01format() {
    let colour = "red";
    let favourite = format!("My favor color is {}", colour);
    println!("{}", favourite);

    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world);

    let favourite_num = format!("My favor numer is {}", 42);
    println!("{}", favourite_num);

    let duck_duck_goose = format!("{0}, {0}, {0}, {1}!", "duck", "goose");
    println!("{}", duck_duck_goose);

    let introduction = format!("My name is {surname}, {forename} {surname}", 
                                surname = "Bond",
                                forename = "James");
    println!("{}", introduction);

    // misc
    println!("{}", format!("{msg} {}", "friendo", msg="Hello there,"));
    println!("{}", format!("{1} {} {0} {}", "a", "b"));
    println!("{:.*}", 2, 1.234);
    }
}

pub mod ch01default {
    #[derive(Default, Debug)]
    struct PizzaConfig {
        wants_cheese: bool,
        number_of_olives: i32,
        special_msg: String,
        crust_type: CrustType,
    }

    #[derive(Debug)]
    enum CrustType {
        Thin,
        Thick,
    }

    impl Default for CrustType {
        fn default() -> CrustType {
            CrustType::Thin
        }
    }
    pub fn ch01default() {
       let foo: i32 = Default::default();
       println!("foo: {}", foo);

       let pizza: PizzaConfig = Default::default();
       println!("wants_cheese: {}", pizza.wants_cheese);
       println!("number_of_olives: {}", pizza.number_of_olives);
       println!("special msg: {}", pizza.special_msg);
       let crust_type = match pizza.crust_type {
           CrustType::Thin => "Nice and thin",
           CrustType::Thick => "Extra thick and extra filling",
       };
       println!("crust_type: {}", crust_type);

       let custom_pizza = PizzaConfig {
           number_of_olives: 12,
           ..Default::default()
       };
       println!("{:#?}", custom_pizza);

       let deluxe_custom_pizza = PizzaConfig {
           number_of_olives: 12,
           wants_cheese: true,
           special_msg: "Will you marry me?".to_string(),
           ..Default::default()
       };
       println!("{:#?}", deluxe_custom_pizza);
    }
}

pub mod ch01constructor {
    struct NameLength {
        name: String,
        len: usize,
    }
    impl NameLength {
        fn new(name: &str) -> Self {
            NameLength {
                name: name.to_string(),
                len: name.len(),
            }
        }
        fn print(&self) {
            println!(
                "The name '{}' is '{}' characters long",
                self.name,
                self.len 
            );
        }
    }
    pub fn ch01constructor() {
        let name_len = NameLength::new("John");
        name_len.print();
        let name_len_cow = NameLen::new("John");
        name_len_cow.print();
    }

    // std::borrow::Cow demo
    struct NameLen<'a> {
        name: std::borrow::Cow<'a, str>,
        len: usize,
    }
    impl<'a> NameLen<'a> {
        fn new<S>(name: S) -> Self where S: Into<std::borrow::Cow<'a, str>>,
        {
            let name: std::borrow::Cow<'a, str> = name.into();
            NameLen {
                len: name.len(),
                name,
            }
        }
        fn print(&self) {
            println!("The name '{}' is '{}' characters long", self.name, self.len);
        }
    }
    pub fn ch01constructor_cow() {
        let name_len_cow_str = NameLength::new("John");
        name_len_cow_str.print();
        let name_len_cow_string = NameLen::new("Alexanda".to_string());
        name_len_cow_string.print();
    }
}

pub mod ch01builder {
    pub fn ch01builder() {

    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new()
                        .cheese(true)
                        .salad(false)
                        .build();
    let veggie_bigmac = BurgerBuilder::new()
                        .vegetarian(true)
                        .patty_count(2)
                        .build();

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }
    if let Ok(veggie_bigmac) = veggie_bigmac {
        veggie_bigmac.print();
    }
    let invalid_burger = BurgerBuilder::new()
                        .vegetarian(true)
                        .bacon(true)
                        .build();
    if let Err(error) = invalid_burger {
        println!("Failed to print burger: {}", error);
    }
    let cheese_burger_builder = BurgerBuilder::new().cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }
    }

    struct Burger {
        patty_count: i32,
        vegetarian: bool,
        cheese: bool,
        bacon: bool,
        salad: bool,
    }
    impl Burger {
        fn print(&self) {
            let pretty_patties = if self.patty_count == 1 {
                "patty"
                } else {
                "patties"
                };
        let pretty_bool = |val| if val { "" } else { "no "};
        let pretty_vegetarian = if self.vegetarian { "vegetarian" } else { "" };
        println!("This is a {}burger with {} {}, {}cheese, {}bacon and {}salad",
                pretty_vegetarian,
                self.patty_count,
                pretty_patties,
                pretty_bool(self.cheese),
                pretty_bool(self.bacon),
                pretty_bool(self.salad)
            )
    }
}

    struct BurgerBuilder {
        patty_count: i32,
        vegetarian: bool,
        cheese: bool,
        bacon: bool,
        salad: bool,
    }
    impl BurgerBuilder {
        fn new() -> Self {
            BurgerBuilder {
                patty_count: 1,
                vegetarian: false,
                cheese: false,
                bacon: false,
                salad: true,
            }
        }
    
    fn patty_count(mut self, val: i32) -> Self {
        self.patty_count = val;
        self
    }
    fn vegetarian(mut self, val: bool) -> Self {
        self.vegetarian = val;
        self
    }
    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val;
        self
    }
    fn bacon(mut self, val: bool) -> Self {
        self.bacon = val;
        self
    }
    fn salad(mut self, val: bool) -> Self {
        self.salad = val;
        self
    }
    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad,
        };
        if burger.vegetarian && burger.bacon {
            Err("Sorry, but we don't server vegetarian bacon yet".to_string())
        } else {
            Ok(burger)
        }
    }
}
}

pub mod ch01parallelism {
    use std::thread;
    pub fn ch01parallelism() {
        let child = thread::spawn(|| println!("Hello from a new thread!"));
        println!("Hello from the main thread!");
        child.join().expect("Failed to join the child thread");
        let sum = parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("The sum of the numbers 1 to 10 is {}", sum);
    }

    fn parallel_sum(range: &[i32]) -> i32 {
        const NUM_THREADS: usize = 4;
        if range.len() < NUM_THREADS {
            sum_bucket(range)
        } else {
            let bucket_size = range.len() / NUM_THREADS;
            let mut count = 0;
            let mut threads = Vec::new();
            while count + bucket_size < range.len() {
                let bucket = range[count..count + bucket_size].to_vec();
                let thread = thread::Builder::new()
                    .name("calculation".to_string())
                    .spawn(move || sum_bucket(&bucket))
                    .expect("Failed to create the thread");
                threads.push(thread);
                count += bucket_size
            }
            let mut sum = sum_bucket(&range[count..]);
            for thread in threads {
                sum += thread.join().expect("Failed to join thread");
            }
            sum
        }
    }
    fn sum_bucket(range: &[i32]) -> i32 {
        let mut sum = 0;
        for num in range {
            //panic!();
            sum += *num;
        }
        sum
    }

}

