/*
Chapter 9 - Project Organization and Structure
https://tourofrust.com/chapter_9_en.html
Modules
Writing a Program
Writing a Library
Referencing Other Modules and Crates
Referencing Multiple Items
Creating Modules
Module Hierarchy
Inline Module
Internal Module Referencing
Exporting
Structure Visibility
Prelude
Your Own Prelude
Chapter 9 - Conclusion
*/
/*
Chapter 9 - Project Organization and Structure
So far all of our code examples have been a single file. Let's discuss how our code can be better organized and shared by others!
*/
/*
Modules
Every Rust program or library is a crate.
Every crate is made of a hierarchy of modules.
Every crate has a root module.
A module can hold global variables, functions, structs, traits or even other modules!
In Rust there is not a 1 to 1 mapping of files to the module tree hierarchy. We must build the module tree explicitly by hand in our code.
*/
/*
Writing a Program
A program has a root module in a file called main.rs.
*/
/*
Writing a Library
A library has a root module in a file called lib.rs
*/
/*
Referencing Other Modules and Crates
Items in modules can be referenced with their full module path std::f64::consts::PI.
A simpler way is the use keyword. It allows us to specify particular items from modules we want to use throughout our code without a full path. For instance use std::f64::consts::PI allows me to just use the identifier PI in my main function.
std is the crate of the standard library of Rust which is full of useful data structures and functions for interacting with your operating system.
A searchable directory of crates created by the community can be found at https://crates.io.
*/
/*
Referencing Multiple Items
Multiple items can be referenced in a single module path as so:
use std::f64::consts::{PI,TAU}
Ferris doesn't eat TAU, he only eats PI.
*/
/*
Creating Modules
When we think of code, we usually imagine a hierarchy of files organized in directories. Rust lets you create modules closely related to your file structure.
There are two ways in Rust to declare a module. For example, a module foo can be represented as:
a file named foo.rs
a directory named foo with a file mod.rs inside
- foo.rs
- foo/mod.rs
*/
/*
Module Hierarchy
A module can depend on another one. In order to establish a relationship between a module and its sub-module, you must write in the parent module:
mod foo;
The declaration above will look for a file named foo.rs or foo/mod.rs and will insert its contents inside a module named foo under this scope.
*/
/*
Inline Module
A sub-module can be directly inlined within a module's code.
One very common use for inline modules is creating unit tests. We create an inline module that only exists when Rust is used for testing!
// This macro removes this inline module when Rust 
// is not in test mode.
#[cfg(test)]
mod tests {
    // Notice that we don't immediately get access to the 
    // parent module. We must be explicit.
    use super::*;
    ... tests go here ...
}
*/
/*
Internal Module Referencing
Rust has several keywords you can use in your use path to quickly get ahold of the module you want:
crate - the root module of your crate
super - the parent module of your current module
self - the current module
*/
/*
Exporting
By default members of a module are not accessible from outside of the module (not even to its child modules!). We make members of a module accessible using the pub keyword.
By default members of a crate are not accessible outside of the crate. We make members of a crate accessible by marking them as pub in the root module of your crate (lib.rs or main.rs).
*/
/*
Structure Visibility
Just like functions, structures can declare what they want exposed outside of their module using pub.
*/
// SeaCreature struct will be usable outside of our module
pub struct SeaCreature {
    pub animal_type: String,
    pub name: String,
    pub arms: i32,
    pub legs: i32,
    // let's keep our weapon private
    weapon: String,
}
/*
Prelude
You might be wondering how we have access to Vec or Box everywhere without a use to import them. It is because of the module prelude in the standard library.
Know that in the Rust standard library anything that is exported in std::prelude::* is automatically available to every part of Rust. That is the case for Vec and Box but others as well (Option, Copy, etc.).
*/
/*
Your Own Prelude
Because of standard library's prelude, it's common for your library to have its own prelude module as a starting point for where users should import all of the most common data structures for using your library (e.g use my_library::prelude::*). It doesn't automatically get used in programs/libraries that use your crate, but it's a good convention to follow so people know where to start.
Ferris says, "Be a good rustacean and help a fellow crab out with a good prelude!"
*/
/*
Chapter 9 - Conclusion
You now have a few tricks up your sleeve when it comes to creating Rust applications and libraries ready for the world. Don't worry about remembering it all. As your library grows and is used by other people, you'll find what works best at each milestone.
Resources:
Guidelines For Writing Rust APIs
https://rust-lang.github.io/api-guidelines/
*/
fn main() {
    dbg!("-- ref other module and crates --");
    use std::f64::consts::PI;
    println!("Welcome to the playground!");
    println!("I would love a slice of {}!", PI);
}

/*
Chapter 10 - The End
It's been a joy to have you on the Tour of Rust. Ferris and the Tour of Rust team sincerely hope you enjoy the journey ahead! If you have felt comfortable this far, we strongly recommend diving deeper with these resources:
The Official Rust Programming Book
https://doc.rust-lang.org/stable/book/
*/