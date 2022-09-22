/*
Chapter 8 - Smart Pointers
https://tourofrust.com/chapter_8_en.html
References Revisited
Raw Pointers
Dereferencing
The * Operator
The . Operator
Smart Pointers
Smart Unsafe Code
Familiar Friends
Heap Allocated Memory
Failable Main Revisited
Referencing Counting
Sharing Access
Sharing Across Threads
Combining Smart Pointers
Chapter 8 - Conclusion
*/
/*
Chapter 8 - Smart Pointers
In this chapter we will demystify smart pointers. Let's explore into these data structures that let us interact with the lowest level of memory.
Ferris says: "Don't feel overwhelmed by this chapter if you don't feel you can write your own low level memory management code in one short read. This chapter is mostly to introduce you to some useful tools and give a glimpse at how they work!"
*/
/*
References Revisited
A reference is fundamentally just a number that is the start position of some bytes in memory. Its only purpose is to represent the concept of where data of a specific type exists. What makes a reference different from just a number is that Rust will validate the lifetime of references doesn't last longer than what it refers to (otherwise we'd get an error when we used it!).
*/
/*
Raw Pointers
References can be converted into a more primitive type called a raw pointer. Much like a number, it can be copied and moved around with little restriction. Rust makes no assurances of the validity of the memory location it points to.
Two kinds of raw pointers exist:
  *const T - A raw pointer to data of type T that should never change.
  *mut T - A raw pointer to data of type T that can change.
Raw pointers can be converted to and from numbers (e.g. usize).
Raw pointers can access data with unsafe code (more on this later).
Memory Details:
  A reference in Rust is very similar to a pointer in C in terms of usage, but with much more compile time restrictions on how it can be stored and moved around to other functions.
  A raw pointer in Rust is similar to a pointer in C that it represents a number that can be copied or passed around, and even turned into numerical types where it can be modifed as a number to do pointer math.
*/
fn raw_ptr() {
    dbg!("-- raw ptr --");
    let a = 42;
    let mem_loc = &a as *const i32 as usize;
    println!("Data is here {:#x}", mem_loc);
    let b = 42u8;
    let mem_loc = &b as *const u8 as usize;
    println!("Data is here {:#x}", mem_loc);
    let c = "c";
    let mem_loc = &c as *const &str as usize;
    println!("Data is here {:#x}", mem_loc);

}
/*
Dereferencing
The process of accessing/manipulating data that is being referred to by a reference (i.e. &i32) is called dereferencing.
References are used to access/manipulate data in two ways:
Access to the referred data during assignment of variables.
Access to fields or methods of the referred data.
Rust has some powerful operators that allow us to do this.
*/
/*
The * Operator
The * operator is an explicit way to dereference a reference.
let a: i32 = 42;
let ref_ref_ref_a: &&&i32 = &&&a;
let ref_a: &i32 = **ref_ref_ref_a;
let b: i32 = *ref_a;
Memory detail:
Because i32 is a primitive type that implements the Copy trait, the bytes of variable a on stack are copied into the bytes of variable b.
*/
fn deref() {
    dbg!("-- deref --");
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("{}", b);
}
/*
The . Operator
The . operator is used in accessing fields and methods of a reference. It works a bit more subtly.
let f = Foo { value: 42 };
let ref_ref_ref_f = &&&f;
println!("{}", ref_ref_ref_f.value);
Whoa, why didn't we need to add *** before ref_ref_ref_f? This is because the . operator automatically dereferences a sequence of references. That last line is turned into the following by the compiler automatically.
println!("{}", (***ref_ref_ref_f).value);
*/
fn dot_op() {
    struct Foo {
        value: i32,
    }
    dbg!("dot op");
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);
}
/*
Smart Pointers
In addition to the ability to create references to existing typed data using the & operator, Rust gives us the ability to create reference-like structs called smart pointers.
We can think of references at a high level as a type that give us access to another type. Smart pointers are different in their behavior from normal references in that they operate based on internal logic that a programmer writes. You — the programmer — are the smart part.
Typically smart pointers implement Deref, DerefMut, and Drop traits to specify the logic of what should happen when the structure is dereferenced with * and . operators.
*/
fn smart_ptr() {
    use std::ops::Deref;
    struct TattleTell<T> {
        value: T,
    }
    impl<T> Deref for TattleTell<T> {
        type Target = T;
        fn deref(&self) -> &T {
            println!("{} was used!", std::any::type_name::<T>());
            &self.value
        }
    }
    dbg!("-- smart ptr --");
    let foo = TattleTell {
        value: "secret msg",
    };
    // dereference occurs here immediately 
    // after foo is auto-referenced for the
    // function `len`
    println!("{}", foo.len());
}
/*
Smart Unsafe Code
Smart pointers tend to use unsafe code fairly often. As mentioned earlier, they are common tools for interacting with the lowest levels of memory in Rust.
What is unsafe code? Unsafe code behaves exactly like normal Rust with the exception of a few abilities that the Rust compiler is unable to make guarantees about.
A primary ability of unsafe code is dereferencing a raw pointer. That means taking a raw pointer to a position in memory and declaring "a data structure exists here!" and turning it into a representation of data you can use (i.e. *const u8 into u8). Rust has no way to keep track of the meaning of every byte that gets written to memory. Because Rust can't make guarantees about what exists at an arbitrary number used as a raw pointer, it puts the dereference in an unsafe { ... } block.
Smart pointers dereference raw pointers extensively, but they are well proven in what they do.
*/
fn smart_unsafe_code() {
    dbg!("-- smart unsafe code --");
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let ptr_a = &a as *const u8 as usize;
    println!("Data mem location: {:#x}", ptr_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let ptr_b = ptr_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *ptr_b
    };
    println!("I swear this is pie! {}", b);
    // extending
    dbg!("f32 -> [u8;4] method 1, via transmute()");
    let pi = 3.1415_f32;
    println!("pi = {}, addr = {:p}", pi, &pi);
    println!("pi = {}, addr = {:p}", &pi, &pi);
    let arr_pi = unsafe { std::mem::transmute::<f32, [u8; 4]>(pi) };
    println!("array pi addr = {:p}", &arr_pi); // copy f32 to new place and transmute to array[u8; 4]
    for i in 0..4 {
        println!("{}", &arr_pi[i]);
    }
    println!("pi = {}, addr = {:p}", &pi, &pi);
    dbg!("f32 -> [u8;4] method 2, via ptr casting and offset()");
    let pi_ptr = &pi as *const f32 as *const u8;
    for i in 0..=3 {
        unsafe { println!("{:?}", *pi_ptr.offset(i)); }
    }
    dbg!("f32 cast to u8 method 3");
    let bits_u32 = pi.to_bits(); // transmute f32 to u32 bitwise
    let bits_u8_ptr = &bits_u32 as *const u32 as *const u8;
    for i in 0..=3 {
        unsafe { println!("{:?}", *bits_u8_ptr.offset(i)); }
    }
}
/*
Familiar Friends
Consider some smart pointers we've already seen like Vec<T> and String.
Vec<T> is a smart pointer that just owns some memory region of bytes. The Rust compiler has no idea what exists in these bytes. The smart pointer interprets what it means to grab items from the region of memory it manages, keeps track of where data structures within those bytes begin and end, and then finally dereferences a raw pointer into data structures into a nice clean ergonomic interface for us to use (e.g. my_vec[3]).
Similarly, String keeps track of a memory region of bytes, and programmatically restricts content written to it to always be valid utf-8 and helps dereference that memory region into a type &str.
Both these datastructures use unsafe dereferencing of raw pointers to do their job.
Memory details:
Rust has an equivalent of C's malloc using alloc and Layout for getting ahold of your own memory regions to manage.
https://doc.rust-lang.org/std/alloc/fn.alloc.html
https://doc.rust-lang.org/std/alloc/struct.Layout.html
*/
fn familiar_friends_alloc_layout() {
    use std::alloc::{alloc, Layout};
    use std::ops::Deref;
    struct Pie {
        secret_recipe: usize,
    }
    impl Pie {
        fn new() -> Self {
            // let's ask for 4 bytes
            let layout = Layout::from_size_align(4, 1).unwrap();
            unsafe {
                // allocate and save the memory location as a number
                let ptr = alloc(layout) as *mut u8;
                // use pointer math and write a few
                // u8 values to memory
                ptr.write(86);
                ptr.add(1).write(14);
                ptr.add(2).write(73);
                ptr.add(3).write(64);
                Pie { secret_recipe: ptr as usize }
            }
        }
    }
    impl Deref for Pie {
        type Target = f32;
        fn deref(&self) -> &f32 {
            // interpret secret_recipe pointer as a f32 raw ptr
            let ptr = self.secret_recipe as *const f32;
            // dereference it into a return value &f32;
            unsafe { &*ptr }
        }
    }
    dbg!("-- familiar friend alloc and layout --");
    let p = Pie::new();
    // "make a pie" by dereferencing our 
    // Pie struct smart pointer
    println!("{:?}", *p);
    dbg!("-- ptr.add() --");
    let s: &str = "123";
    //let ptr: *const u8 = s.as_ptr();
    let ptr = s.as_ptr() as *const u8;
    unsafe {
        println!("{}", *ptr as char);
        println!("{}", *ptr.add(1) as char);
        println!("{}", *ptr.add(2) as char);
        println!("{}", *ptr.add(3) as char);
        println!("{}", *ptr.add(4) as char);
        println!("{}", *ptr.add(5) as char);
        println!("{}", *ptr.add(6) as char);
        println!("{}", *ptr.add(7) as char);
        println!("{}", *ptr.add(8) as char);
        println!("{}", *ptr.add(9) as char);
        println!("{}", *ptr.add(10) as char);
    }
}
/*
Heap Allocated Memory
Box is a smart pointer that lets us move data from the stack to the heap.
Dereferencing it lets us use the heap allocated data ergonomically as if it were the original type.
*/
fn heap_alloc_mem() {
    struct Pie;
    impl Pie {
        fn eat(&self) {
            println!("tastes better on the heap!")
        }
    }
    dbg!("-- heap alloc memory --");
    let heap_pie = Box::new(Pie);
    heap_pie.eat();
}
/*
Failable Main Revisited
Rust code may have a plethora of representations of errors, but the standard library has a universal trait std::error::Error for describing errors.
Using a smart pointer Box we can use the type Box<dyn std::error::Error> as a common type for returning errors because it allows us to propagate up an error on the heap and interact with it at a high level without having to know a specific type.
Early in Tour of Rust we learned that main can return an error. We can now return a type capable of describing almost any kind of error that might occur in our program so long as the error's data structure implements Rust's common Error trait.
fn main() -> Result<(), Box<dyn std::error:Error>>
*/
    use std::fmt::Display;
    use std::error::Error;
fn failable_main_revisited() -> Result<(), Box<dyn Error>>{
    struct Pie;
    #[derive(Debug)]
    struct NotFreshError;
    impl Display for NotFreshError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "This pie is not fresh!")
        }
    }
    impl Error for NotFreshError {}
    impl Pie {
        fn eat(&self) -> Result<(), Box<dyn Error>> {
            Err(Box::new(NotFreshError))
        }
    }
    dbg!("-- failable main revisited --");
    let heap_pie = Box::new(Pie);
    heap_pie.eat()?;
    Ok(())
}
/*
Referencing Counting
Rc is a smart pointer that moves data from the stack onto the heap. It allows us to clone other Rc smart pointers that all have the ability to immutably borrow the data that was put on the heap.
Only when the last smart pointer is dropped does the data on the heap become deallocated.
*/
fn rc() {
    use std::rc::Rc;
    struct Pie;
    impl Pie {
        fn eat(&self) {
            println!("tastes better on the heap!")
        }
    }
    dbg!("-- ref count --");
    let heap_pie = Rc::new(Pie);
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();

    heap_pie3.eat();
    heap_pie2.eat();
    heap_pie.eat();
    // all reference count smart pointers are dropped now
    // the heap data Pie finally deallocates
}
/*
Sharing Access
RefCell is a container data structure commonly held by smart pointers that takes in data and lets us borrow mutable and immutable references to what's inside. It prevents borrowing from being abused by enforcing Rust's memory safety rules at runtime when you ask to borrow the data within:
Only one mutable reference OR multiple immutable references, but not both!
If you violate these rules RefCell will panic.
*/
fn sharing_access() {
    use std::cell::RefCell;
    struct Pie {
        slices: u8
    }
    impl Pie {
        fn eat(&mut self) {
            println!("tastes better on the heap!");
            self.slices -= 1;
        }
    }
    dbg!("-- sharing access --");
    // RefCell validates memory safety at runtime
    // notice: pie_cell is not mut!
    let pie_cell = RefCell::new(Pie{slices: 8});
    {
        let mut mut_ref_pie = pie_cell.borrow_mut(); // // but we can borrow mutable references!
        mut_ref_pie.eat();
        mut_ref_pie.eat(); // mut_ref_pie is dropped at end of scope
    }
        // now we can borrow immutably once our mutable reference drops
    let ref_pie = pie_cell.borrow();
    println!("{} slices left", ref_pie.slices);
}
/*
Sharing Across Threads
Mutex is a container data structure commonly held by smart pointers that takes in data and lets us borrow mutable and immutable references to the data within. This prevents borrowing from being abused by having the operating system restrict only one CPU thread at time to have access to the data, blocking other threads until that original thread is done with its locked borrow.
Multithreading is beyond the scope of Tour of Rust, but Mutex is a fundamental part of orchestrating multiple CPU threads accessing the same data.
There is a special smart pointer Arc which is identical to Rc except uses thread-safe incrementing of reference counts. It's often used to have many references to the same Mutex.
*/
fn sharing_across_threads() {
    use std::sync::Mutex;
    struct Pie;
    impl Pie {
        fn eat(&self) {
            println!("only I eat the pie right now!");
        }
    }
    dbg!("-- sharing across threads --");
    let mutex_pie = Mutex::new(Pie);
    // let's borrow a locked immutable reference of pie
    // we have to unwrap the result of a lock
    // because it might fail
    let ref_pie = mutex_pie.lock().unwrap();
    ref_pie.eat();
    // locked reference drops here, and mutex protected value can be used by someone else
}
/*
Combining Smart Pointers
Smart pointers might seem limited, but they can make some very powerful combinations.
Rc<Vec<Foo>> - Allow the cloning of multiple smart pointers that can borrow the same vector of immutable data structures on the heap.
Rc<RefCell<Foo>> - Allow multiple smart pointers the ability to borrow mutably/immutably the same struct Foo
Arc<Mutex<Foo>> - Allow multiple smart pointers the ability to lock temporary mutable/immutable borrows in a CPU thread exclusive manner.
Memory detail:
You'll notice a theme with many of these combinations. The use of a immutable data type (possibly owned by multiple smart pointers) to modify internal data. This is referred to as the "interior mutability" pattern in Rust. It is a pattern that lets us bend the rules of memory usage at runtime with the same level of safety as Rust's compile-time checks.
*/
fn combine_smart_ptrs() {
    use std::cell::RefCell;
    use std::rc::Rc;
    struct Pie { slices: u8, }
    impl Pie {
        fn eat_slice(&mut self, name: &str) {
            println!("{} took a slice!", name);
            self.slices -= 1;
        }
    }
    struct SeaCreature {
        name: String,
        pie: Rc<RefCell<Pie>>,
    }
    impl SeaCreature {
        fn eat(&self) {
            // use smart pointer to pie for a mutable borrow
            let mut p = self.pie.borrow_mut();
            // take a bite
            p.eat_slice(&self.name);
        }
    }
    dbg!("-- combile smart ptrs --");
    let pie = Rc::new(RefCell::new(Pie {slices: 8}));
        // ferris and sarah are given clones of smart pointer to pie
    let ferris = SeaCreature {
        name: String::from("ferris"),
        pie: pie.clone(),
    };
    let sarah = SeaCreature {
        name: String::from("sarah"),
        pie: pie.clone(),
    };
    ferris.eat();
    sarah.eat();

    let p = pie.borrow();
    println!("{} slices left", p.slices);
}
/*
Chapter 8 - Conclusion
Smart pointers are the idioms of Rust programming and let us not have to re-create the very common patterns of memory usage. With them you are ready to tackle the toughest of challenges! Now that we have the foundations of Rust, let's talk a bit about how we make larger projects. In chapter 9 we break free of single page lines of code.
*/
fn main() {
    raw_ptr();
    deref();
    dot_op();
    smart_ptr();
    smart_unsafe_code();
    familiar_friends_alloc_layout();
    heap_alloc_mem();
    match failable_main_revisited() {
        Ok(()) => {},
        Err(e) => { println!("{:?}", e) },
    }
    rc();
    sharing_access();
    sharing_across_threads();
    combine_smart_ptrs();
}