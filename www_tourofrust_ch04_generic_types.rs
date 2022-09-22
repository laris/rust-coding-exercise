/*
Chapter 4 - Generic Types
Generic types are incredibly important in Rust. They are used in the representation of nullable values (i.e. variables which might not have a value yet), error handling, collections, and more! In this section we will be learning about the foundational generic types you will likely be using all the time.
https://tourofrust.com/chapter_4_en.html
*/

/*
What Are Generic Types?
Generic types allow us to partially define a struct or enum, enabling a compiler to create a fully defined version at compile-time based off our code usage.
Rust generally can infer the final type by looking at our instantiation, but if it needs help you can always be explicit using the ::<T> operator, also known by the name turbofish (he's a good friend of mine!).
*/

fn what_are_generic_types() {
// A partially defined struct type
struct BagOfHolding<T> {
    item: T,
}

    dbg!("-- What Are Generic Types? --");
    // Note: by using generic types here, we create compile-time created types. 
    // Turbofish lets us be explicit.
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };
    
    // Rust can infer types for generics too!
    let float_bag = BagOfHolding { item: 3.14 };
    
    // Note: never put a bag of holding in a bag of holding in real life
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );    
}

/*
Representing Nothing

In other languages, the keyword null is used to represent an absence of a value. It creates difficulty in programming languages because it creates the possibility that our program might fail when interacting with a variable/field.

Rust does not have null, but it is not ignorant of the importance of representing nothing! Consider a naive representation using a tool we already know.

This pattern of providing a None alternative representation for one or many alternate values is so common in Rust because of its lack of a null value. Generic types help solve this challenge.
*/
fn represent_nothing() {
    enum Item {
        Inventory(String),
        // None represents the absence of an item
        None,
    }
    
    struct BagOfHolding {
        item: Item,
    }
}

/*
Option

Rust has a built in generic enum called Option that allows us to represent nullable values without using null.

enum Option<T> {
    None,
    Some(T),
}
This enum is so common, instances of the enum can be created anywhere with the enum variants Some and None.
*/
fn option() {
// A partially defined struct type
struct BagOfHolding<T> {
    // Our parameter type T can be handed to others
    item: Option<T>,
}

//fn main() {
    dbg!("-- Option --");
    // Note: A bag for i32, holding nothing! We have to specify the type
    // because otherwise Rust would not know what type of bag it is.
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!")
    } else {
        println!("there's something in the bag!")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("there's something in the bag!")
    } else {
        println!("there's nothing in the bag!")
    }

    // match lets us deconstruct Option elegantly and ensure we handle all cases!
    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }    
}
/*
Result

Rust has a built in generic enum called Result that allows us to return a value that has the possibility of failing. It is the idiomatic way in which the language does error handling.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
Note that our generics type has multiple parameterized types separated by a comma.

This enum is so common, instances of the enum can be created anywhere with the enum variants Ok and Err.
*/
fn result() {
    fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
        if i == 42 {
            Ok(13.0)
        } else {
            Err(String::from("this is not the right number"))   
        }
    }
    
//    fn main() {
    dbg!("-- Result --");
        let result = do_something_that_might_fail(12);
    
        // match lets us deconstruct Result elegantly and ensure we handle all cases!
        match result {
            Ok(v) => println!("found {}", v),
            Err(e) => println!("Error: {}",e),
        }

}
/*
Failable Main
main has the capability of returning a Result!
*/
fn failable_main() -> Result<(), String> {
    fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.0)
        } else {
            Err(String::from("this is not the right number"))
        }
    }
    dbg!("-- Failable Main --"); 
    //fn main() -> Result<(), String> {
        let result = do_something_that_might_fail(12);
        match result {
            Ok(v) => println!("found {}", v),
            Err(_e) => {
                return Err(String::from("something went wrong in main!"));
            }
        }
        Ok(())    
}
/*
Graceful Error Handling
Result is so common that Rust has a powerful operator ? for working with them. These two statements are equivalent:
do_something_that_might_fail()?
match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
*/
fn err_graceful_handling() -> Result<(), String> {
    fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.0)
        } else {
            Err(String::from("this is not the right number"))
        }
    }
    dbg!("-- Graceful Err Handling --"); 
    //fn main() -> Result<(), String> {
        // Look at how much code we saved!
        let v = do_something_that_might_fail(42)?;
        println!("found {}", v);
        Ok(())    
}

/*
Ugly Option/Result Handling

Working with Option/Result can be tedious when you are just trying to write some quick code. Both Option and Result have a function called unwrap that can be useful for getting a value in a quick and dirty manner. unwrap will:

Get the value inside Option/Result
If the enum is of type None/Err, panic!
These two pieces of code are equivalent:

my_option.unwrap()
match my_option {
    Some(v) => v,
    None => panic!("some error message generated by Rust!"),
}
Similarly:

my_result.unwrap()
match my_result {
    Ok(v) => v,
    Err(e) => panic!("some error message generated by Rust!"),
}
Be a good rustacean and properly use match when you can!
*/
fn ugly_option_result_handling() -> Result<(), String> {
    fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
        if i == 42 {
            Ok(13.0)
        } else {
            Err(String::from("this is not the right number"))
        }
    }
    dbg!("-- ugly option result handling --");
    //fn main() -> Result<(), String> {
        // concise but assumptive and gets ugly fast
        let v = do_something_that_might_fail(42).unwrap();
        println!("found {}", v);
        
        // this will panic!
        //let v = do_something_that_might_fail(1).unwrap();
        //println!("found {}", v);
        
        Ok(())
}
/*
Vectors
Some of the most useful generic types are collection types. A vector is a variably sized list of items represented by the struct Vec.
The macro vec! lets us easily create a vector rather than manually constructing one.
Vec has the method iter() which creates an iterator from a vector, allowing us to easily put a vector into a for loop.
Memory Details:
Vec is a struct, but internally it contains a reference to a fixed list of its items on the heap.
A vector starts with a default capacity; when more items are added than it has capacity for, it reallocates its data on the heap to have a new fixed list with large capacity.
*/
fn vector() {
    dbg!("-- Vector --");
    // We can be explicit with type
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // But look how clever Rust is about determining the type automatically
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // That's a beautiful macro!
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }    
}

fn main() {
    what_are_generic_types();
    option();
    result();
    match failable_main() {
        Ok(()) => {},
        Err(e) => println!("{:?}", e),
    }
    match err_graceful_handling() {
        Ok(()) => {},
        Err(e) => println!("{:?}", e),
    }
    match ugly_option_result_handling() {
        Ok(()) => {},
        Err(e) => println!("{:?}", e),
    }
    vector();
}

/*
Chapter 4 - Conclusion

In one chapter we've learned how much power generic types give us! Don't worry if you don't know fully how to use everything, right now it's just good to be aware of the major ideas you will see again and again in code. Our functions are getting quite lengthy! In our next chapter we will spend talk about an important concept in Rust: data ownership.
*/