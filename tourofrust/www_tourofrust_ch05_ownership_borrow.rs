// Chapter 5 - Ownership & Borrowing Data
// https://tourofrust.com/chapter_5_en.html
/*
Ownership
Scope-Based Resource Management
Dropping is Hierarchical
Moving Ownership
Returning Ownership
Borrowing Ownership with References
Borrowing Mutable Ownership with References
Dereferencing
Passing Around Borrowed Data
References Of References
Explicit Lifetimes
Multiple Lifetimes
Static Lifetimes
Lifetimes In Data Types
Chapter 5 - Conclusion
*/

/*
Chapter 5 - Ownership & Borrowing Data
Rust has a unique paradigm for managing memory compared to other programming languages.
We're going to look at the behaviors and validations of the compiler one by one so it's not overwhelming.
It's important to remember that ultimately the rules we show don't exist to make your life hard, 
but to help you make your code less error-prone!
*/

/*
Ownership
Instantiating a type and binding it to a variable name creates a memory resource that the Rust compiler will validate through its whole lifetime. The bound variable is called the resource's owner.
*/
struct Foo {
    x: i32,
}
fn ownership() {
    dbg!("-- ownership --");
    // We instantiate structs and bind to variables
    // to create memory resources
    let foo = Foo { x: 42 };
    // foo is the owner
}
/*
Scope-Based Resource Management
Rust uses the end of scope as the place to deconstruct and deallocate a resource.
The term for this deconstruction and deallocation is called a drop.
Memory detail:
Rust does not have garbage collection.
This is also called Resource Acquisition Is Initialization ( RAII ) in C++.
*/
fn scope_based_resource_manage() {
    dbg!("-- scope based resource manage");
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);

    println!("{}", foo_b.x);
    // foo_b is dropped here 
    // foo_a is dropped here
}

/*
Dropping is Hierarchical
When a struct is dropped, the struct itself is dropped first, then its children are dropped individually, and so on.
Memory Details:
By automatically freeing memory Rust helps ensure that there are fewer memory leaks.
Memory resources can only be dropped once.
*/
struct Bar {
    x: i32,
}

struct Foo2 {
    bar: Bar,
}
fn drop_hierarchical() {
    dbg!("-- drop hierarchical --");
    let foo = Foo2 { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo is dropped first
    // then foo.bar is dropped
}

/*
Moving Ownership
When an owner is passed as an argument to a function, ownership is moved to the function parameter.
After a move the variable in the original function can no longer be used.
Memory details:
During a move the stack memory of the owners value is copied to the function call's parameter stack memory.
*/
fn do_something(f: Foo) {
    println!("{}", f.x); // f drop here
}
fn move_ownership() {
    dbg!("-- move ownership -- ");
    let foo = Foo { x: 42 };
    // foo is moved to do_something
    do_something(foo);
    // foo can no longer be used
}
/*
Returning Ownership
Ownership can also be returned from a function.
*/
fn return_ownership() {
    #[derive(Debug)]
    struct Foo {
        x: i32,
    }
    
    fn do_something() -> Foo {
        Foo { x: 42 }
        // ownership is moved out
    }
        dbg!("-- return ownership --");
        let foo = do_something();
        println!("{:?}", foo);
        // foo becomes the owner
        // foo is dropped because of end of function scope
}

/*
Borrowing Ownership with References
References allow us borrow access to a resource with the & operator.
References are also dropped like other resources.
*/
fn borrow_with_ref() {
    struct Foo {
        x: i32,
    }

    dbg!("-- borrow with ref -- ");
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f is dropped here
    // foo is dropped here
}

/*
Borrowing Mutable Ownership with References
We can also borrow mutable access to a resource with the &mut operator.
A resource owner cannot be moved or modified while mutably borrowed.
Memory detail:
Rust prevents having two ways to mutate an owned value because it introduces the possibility of a data race.
*/
fn borrow_mut_with_ref() {
    struct Foo {
        x: i32,
    }
    
    fn do_something(f: Foo) {
        println!("{}", f.x);
        // f is dropped here
    }
    
    dbg!("-- borrow mut with ref");
        let mut foo = Foo { x: 42 };
        let f = &mut foo;
    
        // FAILURE: do_something(foo) would fail because
        // foo cannot be moved while mutably borrowed
    
        // FAILURE: foo.x = 13; would fail here because
        // foo is not modifiable while mutably borrowed
    
        f.x = 13;
        // f is dropped here because it's no longer used after this point
        
        println!("{}", foo.x);
        
        // this works now because all mutable references were dropped
        foo.x = 7;
        
        // move foo's ownership to a function
        do_something(foo);
}

/*
Dereferencing
Using &mut references, you can set the owner's value using the * operator.
You can also get a copy of an owned value using the * operator (if the value can be copied - we will discuss copyable types in later chapters).
*/
fn deref() {
    dbg!("-- deref");
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13;      // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);    
}

/*
Passing Around Borrowed Data
Rust's rules for references might best be summarized by:
Rust only allows there to be one mutable reference or multiple non-mutable references but not both.
A reference must never live longer than its owner.
This doesn't tend to be a problem when passing around references to functions.
Memory details:
The first rule of references prevents data races. What's a data race? A data race when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
The second rule of references prevents the misuse of references that refer to non-existent data (called dangling pointers in C).
*/
fn pass_around_borrowed_data() {
    struct Foo {
        x: i32,
    }
    
    fn do_something(f: &mut Foo) {
        f.x += 1;
        // mutable reference f is dropped here
    }
    dbg!("-- pass around borrowed data"); 
        let mut foo = Foo { x: 42 };
        do_something(&mut foo);
        // because all mutable references are dropped within
        // the function do_something, we can create another.
        do_something(&mut foo);
        // foo is dropped here
}
/*
References Of References
References can even be used on pieces of references.
*/
fn ref_of_ref() {
    struct Foo {
        x: i32,
    }
    
    fn do_something(a: &Foo) -> &i32 {
        return &a.x;
    }
   
    dbg!("-- ref_of_ref");
        let mut foo = Foo { x: 42 };
        let x = &mut foo.x;
        *x = 13;
        // x is dropped here allow us to create a non-mutable reference
        let y = do_something(&foo);
        println!("{}", y);
        // y is dropped here
        // foo is dropped here    
}
/*
Explicit Lifetimes
Even though Rust doesn't always show it in code, the compiler understands the lifetime of every variable and will attempt to validate that a reference never exists longer than its owner.
Functions can be explicit by parameterizing the function signature with symbols that help identify which parameters and return values share the same lifetime.
Lifetime specifiers always start with a ' (e.g. 'a, 'b, 'c)
*/
fn explicit_lifetimes() {
    struct Foo {
        x: i32,
    }
    
    // the parameter foo and return value share the same lifetime
    fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
        return &foo.x;
    }
    dbg!("-- explicit lifetime"); 
        let mut foo = Foo { x: 42 };
        let x = &mut foo.x;
        *x = 13;
        // x is dropped here, allowing us to create a non-mutable reference
        let y = do_something(&foo);
        println!("{}", y);
        // y is dropped here
        // foo is dropped here
}
/*
Multiple Lifetimes
Lifetime specifiers allow us to be explicit with certain scenarios the compiler cannot resolve itself by distinguishing all of a function signature component's lifetimes.
*/
fn multiple_lifetimes() {
    struct Foo {
        x: i32,
    }
    
    // foo_b and the return value share the same lifetime
    // foo_a has an unrelated lifetime
    fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
        println!("{}", foo_a.x);
        println!("{}", foo_b.x);
        return &foo_b.x;
    }
    
        dbg!("-- Multiple lifetimes --");
        let foo_a = Foo { x: 42 };
        let foo_b = Foo { x: 12 };
        let x = do_something(&foo_a, &foo_b);
        // foo_a is dropped here because only foo_b's lifetime exist beyond here
        println!("{}", x);
        // x is dropped here
        // foo_b is dropped here
}
/*
Static Lifetimes
A static variable is a memory resource created at compile-time that exists through a program start to finish. They must have their types explicitly specified.
A static lifetime is a memory resource that lasts indefinitely to the end of a program. Note that by this definition some static lifetime resources can be created at runtime.
Resources with static lifetimes have a special lifetime specifier 'static.
'static resources will never drop.
If static lifetime resources contain references they must all be 'static (anything less would not live long enough).
Memory detail:
Modifying static variables is inherently dangerous because they are globally accessable to be read from by anyone introducing the possibility of a data race. We'll talk about the challenges of global data later.
Rust allows the use of unsafe { ... } blocks to perform some operations that the compiler cannot make memory guarantees about. The 

R̸͉̟͈͔̄͛̾̇͜U̶͓͖͋̅Ṡ̴͉͇̃̉̀T̵̻̻͔̟͉́͆Ơ̷̥̟̳̓͝N̶̨̼̹̲͛Ö̵̝͉̖̏̾̔M̶̡̠̺̠̐͜Î̷̛͓̣̃̐̏C̸̥̤̭̏͛̎͜O̶̧͚͖͔̊͗̇͠N̸͇̰̏̏̽̃ 

should not be talked about casually.
*/
fn static_lifetimes() {
    static PI: f64 = 3.1415;

        dbg!("-- static lifetime --");
        // static variables can also be scoped to a function
        static mut SECRET: &'static str = "swordfish";
    
        // string literals have a 'static lifetime
        let msg: &'static str = "Hello World!";
        let p: &'static f64 = &PI;
        println!("{} {}", msg, p);
    
        // You can break some rules, but you must be explicit
        unsafe {
            // we can set SECRET to a string literal because it is also `static
            SECRET = "abracadabra";
            println!("{}", SECRET);
        }    
}
/*
Lifetimes In Data Types
Similarly to functions, data types can be parameterized with lifetime specifiers of its members.
Rust validates that the containing data structure of the references never lasts longer than the owners its references point to.
We can't have structs running around with references pointing to nothingness!
*/
fn lifetimes_in_data_types() {
    struct Foo<'a> {
        i:&'a i32
    }
    dbg!("-- lifetime in data types");
        let x = 42;
        let foo = Foo {
            i: &x
        };
        println!("{}",foo.i);    
}
//------
fn main() {
    ownership();
    scope_based_resource_manage();
    drop_hierarchical();
    move_ownership();
    return_ownership();
    borrow_with_ref();
    borrow_mut_with_ref();
    deref();
    ref_of_ref();
    explicit_lifetimes();
    multiple_lifetimes();
    static_lifetimes();
    lifetimes_in_data_types();
}
/*
Chapter 5 - Conclusion

Whew, congrats for making it through! I know it's a lot to take in, but you are well under way to becoming a Rustacean. Hopefully it's clear how Rust as a language aims to solve many of these common challenges in systems programming:

Unintentional modification of resources
Forgetting to deconstruct resources
Resources accidentally being deconstructed twice
Using resources after they have been deconstructed
Data races caused by writing to resources while others are reading from resources
Seeing clearly areas of the code where the compiler can’t make guarantees
In the next chapter we'll apply some of this knowledge as we look at how Rust handles text.
*/