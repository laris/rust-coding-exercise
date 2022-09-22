/*
Chapter 7 - Object Oriented Programming
https://tourofrust.com/chapter_7_en.html

What Is OOP?
Rust Is Not OOP
Encapsulation With Methods
Abstraction With Selective Exposure
Polymorphism With Traits
Implemented Methods On Traits
Trait Inheritance
Dynamic vs Static Dispatch
Trait Objects
Handling Unsized Data
Generic Functions
Generic Function Shorthand
Box
Generic Structs Revisited
Chapter 7 - Conclusion
*/

/*
Chapter 7 - Object Oriented Programming
Expressing ideas with functions is a proven way of representing behavior and data (C has been doing it for decades!). Historically, computer science has found other useful expressive aggregations and abstractions for data. You may be familiar with object oriented programming (OOP) as one such way. In this chapter we'll explore the Rust programming language beyond functions.
*/
/*
What Is OOP?
Object oriented programming roughly refers to programming languages that have a number of iconic features:
Encapsulation - Associating data and functions into the conceptual unit of a single type called an object.
Abstraction - Hiding data and function members to obfuscate implementation details of an object.
Polymorphism - The ability to interact with objects of different types through one interface.
Inheritance - The ability to inherit data and behavior from other objects.
*/
/*
Rust Is Not OOP
Rust lacks inheritance of data and behavior in any meaningful way.
Structs cannot inherit fields from a parent struct.
Structs cannot inherit functions from a parent struct.
That said, Rust implements many programming language features, so that you might not mind this lacking.
*/
/*
Encapsulation With Methods
Rust supports the concept of an object that is a struct associated with some functions (also known as methods).
The first parameter of any method must be a reference to the instance associated with the method call (e.g. instanceOfObj.foo()). Rust uses:
&self - Immutable reference to the instance.
&mut self - Mutable reference to the instance.
Methods are defined within an implementation block with keyword impl:
impl MyStruct { 
    ...
    fn foo(&self) {
        ...
    }
}
*/
fn encap_with_method() {
    struct SeaCreature {
        noise: String,
    }
    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    dbg!("encap_with_method");
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}
/*
Abstraction With Selective Exposure
Rust can hide the inner workings of objects.
By default fields and methods are accessible only to the module they belong to.
The pub keyword exposes struct fields and methods outside of the module.
*/
fn abs_with_selective_exposure() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    dbg!("abstraction with selective exposure");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
    println!("{}", creature.name);
    println!("{}", creature.noise);
}
/*
Polymorphism With Traits
Rust supports polymorphism with traits. Traits allow us to associate a set of methods with a struct type.
We first define the signatures of methods of a trait within:
trait MyTrait {
    fn foo(&self);
    ...
}
When a struct implements a trait, it establishes a contract that allows us to indirectly interact with the struct through the trait type (e.g. &dyn MyTrait) without having to know the real type.
A struct's implemented traits methods are defined within an implementation block:
impl MyTrait for MyStruct { 
    fn foo(&self) {
        ...
    }
    ... 
}
*/
fn polymorphism() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    trait NoiseMaker {
        fn make_noise(&self);
    }
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    dbg!("-- polymorphism --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
}
/*
Implemented Methods On Traits
Traits can have implemented methods.
The functions have no direct access to the inner fields of a struct, but it can be useful for sharing behavior between many trait implementors.
*/
fn imp_method_on_traits() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    trait NoiseMaker {
        fn make_noise(&self);
        fn make_alot_of_noise(&self) {
            self.make_noise();
            self.make_noise();
            self.make_noise();
        }
    }
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    dbg!("-- impl method on traits --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_alot_of_noise();
}
/*
Trait Inheritance
Traits can inherit methods from other traits.
*/
fn trait_inherit() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    trait NoiseMaker {
        fn make_noise(&self);
    }
    trait LoudNoiseMaker: NoiseMaker {
        fn make_alot_of_noise(&self) {
            self.make_noise();
            self.make_noise();
            self.make_noise();
        }
    }
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    impl LoudNoiseMaker for SeaCreature {}
    dbg!("-- trait inherit --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_alot_of_noise();
}
/*
Dynamic vs Static Dispatch
Methods are executed in two ways:
static dispatch - When the instance type is known, we have direct knowledge of what function to call.
dynamic dispatch - When an instance type is not known, we must find out some way of calling the correct function.
Trait types &dyn MyTrait give us the ability to work with instances of objects indirectly using dynamic dispatch.
When dynamic dispatch is used, Rust will encourage you to put dyn before your trait type so people are aware.
Memory details:
Dynamic dispatch is slightly slower because of the pointer chasing to find the real function call.
*/
fn dyn_static_dispatch(){
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    
    trait NoiseMaker {
        fn make_noise(&self);
    }
    
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    fn static_make_noise(creature: &SeaCreature) {
        // we knoow the real type
        creature.make_noise();
    }
    fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
        // we don't know the real type
        noise_maker.make_noise();
    }
    fn imp_make_noise(noise_maker: &impl NoiseMaker) {
        noise_maker.make_noise();
    }
    fn trait_bound_make_noise<T: NoiseMaker>(noise_maker: &T) {
        noise_maker.make_noise();
    }
    dbg!("-- dyn vs static --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
    trait_bound_make_noise(&creature);
    imp_make_noise(&creature);
}
/*
Trait Objects
When we pass an instance of an object to a parameter of type &dyn MyTrait we pass what is called a trait object.
A trait object is what allows us to indirectly call the correct methods of an instance. A trait object is a struct that holds the pointer of our instance with a list of function pointers to our instance's methods.
Memory details:
This list of functions is known in C++ as a vtable.
*/
/*
Handling Unsized Data
Traits introduce an interesting challenge when we want to store them within another struct. Traits obfuscate the original struct thus it also obfuscates the original size. Unsized values being stored in structs are handled in two ways in Rust:
generics - Using parameterized types effectively create struct/functions known types and thus known sizes.
indirection - Putting instances on the heap gives us a level of indirection that allow us to not have to worry about the size of the actual type and just store a pointer to it. There are other ways as well!
*/
/*
Generic Functions
Generics in Rust work hand in hand with traits. When we describe a parameterized type T we can constrain what types can be used as an argument by listing what required traits the argument must implement.
In this example type T must implement trait Foo:
fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}
By using generics we create static typed functions at compile time that will have known types and sizes, allowing us to perform static dispatch and store as a sized value.
*/
fn unsized_generic_func() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    
    trait NoiseMaker {
        fn make_noise(&self);
    }
    
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    fn generic_make_noise<T>(creature: &T)
        where T: NoiseMaker
        {
            // we know the real type at compile-time
            creature.make_noise();
        }
    dbg!("-- unsized generic function --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise(&creature);
}
/*
Generic Function Shorthand
Rust has a shorthand for expressing generics constrained by a trait:
fn my_function(foo: impl Foo) {
    ...
}
This is equivalent to writing:
fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}
*/
fn unsized_generic_func_shorthand() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    
    trait NoiseMaker {
        fn make_noise(&self);
    }
    
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    fn generic_make_noise(creature: &impl NoiseMaker) {
        // we know the real type at compile-time
        creature.make_noise();
    }
    dbg!("-- unsized_generic_func_shorthand --");
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise(&creature);
}
/*
Box
Box is a data structure that allows us to move our data from the stack to the heap.
Box is a struct known as a smart pointer that holds the pointer to our data on the heap.
Because Box is a struct with a known size (because it just holds a pointer), it is often used as a way to store a reference to something in a struct that must know the size of its fields.
Box is so common it can be used from anywhere:
Box::new(Foo { ... })
*/
fn unsized_box() {
    struct SeaCreature {
        pub name: String,
        noise: String,
    }
    
    impl SeaCreature {
        pub fn get_sound(&self) -> &str {
            &self.noise
        }
    }
    
    trait NoiseMaker {
        fn make_noise(&self);
    }
    
    impl NoiseMaker for SeaCreature {
        fn make_noise(&self) {
            println!("{}", &self.get_sound());
        }
    }
    struct Ocean {
        animals: Vec<Box<dyn NoiseMaker>>,
    }    
    dbg!("-- unsized box --");
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };
    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };
    for a in ocean.animals.iter() {
        a.make_noise();
    }
}
/*
Generic Structs Revisited
Generic structs can also have their parameterized types constrained by traits.
struct MyStruct<T>
where
    T: MyTrait
{
    foo: T
    ...
}
Generic structs have their parameterized type in their implementation blocks:
impl<T> MyStruct<T> {
    ...
}
*/
/*
Chapter 7 - Conclusion
We now have more language features at hand to represent our ideas clearly! Rust abstractions might be simple but they are powerful enough to make working with code a joy. In this chapter, we caught a glimpse of smart pointers with Box. In the next chapter we'll learn about how smart pointers can help us with other specialized memory situations.
Resources:
Video - Object-oriented Programming in 7 minutes
https://www.youtube.com/watch?v=pTB0EiLXUC8
Article - "The faster you unlearn OOP, the better for you and your software"
https://dpc.pw/the-faster-you-unlearn-oop-the-better-for-you-and-your-software
*/
fn main() {
    encap_with_method();
    abs_with_selective_exposure();
    polymorphism();
    imp_method_on_traits();
    trait_inherit();
    dyn_static_dispatch();
    unsized_generic_func();
    unsized_generic_func_shorthand();
    unsized_box();
}