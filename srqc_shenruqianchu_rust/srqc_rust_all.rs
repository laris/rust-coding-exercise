
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
    //ch1_1_helloworld();
    //ch1_5_println_fmt();
    //ch2_1_var();
    //ch7_2_2();
    //ch7_2_3();
    //ch7_2_4();
    //ch23_0::ch23_0_static_dynamic_dispatch();
    //ch23_1::ch23_1_trait_object();
}

fn ch1_1_helloworld() {
    let s = "Hellow world!";
    println!("{}", s);
}

fn ch1_5_println_fmt() {
    println!("dec: {}", 1);
    println!("oct: {:o}", 9);
    println!("hex: {:x}", 255);
    println!("HEX: {:X}", 255);
    println!("bin: {:b}", 255);
    println!("ptr: {:p}", &0);
    println!("{:e}", 1000_f32);
    println!("{:E}", 1000_f64);
    println!("dbg: {:?}", "test");
    println!("dbg: {:#?}", ("t1", "t2"));
    println!("arg: {a} {b}\t{c}", a = "x", b = "y", c = "z");
}

fn ch2_1_var() {
    // declare
    let variable: i32 = 100;

    // immutable
    let x = 5;
    //x = 10;

    // mutable
    let mut y = 5;
    y = 5;

    // pattern
    let (mut a, mut b) = (1, 2);

    struct Point { x: i32, y: i32 }
    let p = Point { x: 0, y: 0 };
    let Point { x: ref a, y: ref b } = p;
    println!("Point x: {}", x);

    // initiliazation
    let z: i32;
    // println!("{}", z); 
    // error[E0381]: borrow of possibly-uninitialized variable: `z`

}


// 7-2-2
struct P(f32, f32, f32);

fn calc(arg: P) -> f32 {
    let P(x, _, y) = arg;
    x * x + y * y
}

fn calc_2(P(x, _, y): P) -> f32 {
    x * x + y * y
}

fn ch7_2_2() {
    let t = P(1.0, 2.0, 3.0);
    let t_2 = P(1.0, 2.0, 3.0);
    println!("{}", calc(t));
    println!("{}", calc_2(t_2));

    // error
    let x = 1;
    // let _ = 1_i32;
    // let x = _ + _; // error: in expressions, `_` can only be used on the left-hand side of an assignment
    println!("{}", x);

    // _ cannot replace more than one value
    let x = (1, 2, 3);
    //let (a, _) = x; //  expected a tuple with 3 elements, found one with 2 elements
    let (a, _, _) = x;
    println!("{}", a);
    let (b, ..) = x;
    println!("{}", b);
    let (c, .., d) = x;
    println!("{} {}", c, d);
}

//ch7_2_3();
enum Direction {
    East, West, South, North, 
}

fn direction_to_int(x: Direction) -> i32 {
    match x {
        Direction::East     => 10,
        Direction::West     => 20,
        Direction::South    => 30,
        Direction::North    => 40,
    }
}

fn category(x: i32) {
    match x {
        -1 => println!("negative"),
         0 => println!("zero"),
         1 => println!("positive"),
         _ => println!("error"),
    }
}

fn category_2(x: i32) {
    match x {
        -1 | 1 => println!("true"),
             0 => println!("false"),
             _ => println!("error"),
    }
}

fn ch7_2_3() {
    let x = Direction::East;
    let s = direction_to_int(x);
    println!("{}", s);

    let x = 1;
    category(x);
    category_2(x);

    let y = 'Y';
    match y {
        'a' ..= 'z' => println!("lowercase"),
        'A' ..= 'Z' => println!("uppercase"),
                  _ => println!("something else"),
    }
}


//    ch7_2_4();
enum OptionalInt {
    Value(i32),
    Missing,
}
fn ch7_2_4() {
    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    let y = 10;
    match y {
        i if i > 5 => println!("bigger than five"),
        i if i <= 5 => println!("small or equal to five"),
        _ => unreachable!(), //   ^ pattern `_` not covered
    }

    let z = -1;
    match z {
        i if i < 0          => println!("case 1"),
        i if i < 10         => println!("case 2"),
        i if i * i < 100    => println!("case 3"),
        _                   => println!("default case"),
    }
}

//fn ch23_0_static_dynamic_dispatch()
mod ch23_0 {

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) { println!("duck duck"); }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan");
    }
}


fn test_dispatch_static<T: Bird>(arg: T) {
    arg.fly();
}

fn test_dispatch_dyn(arg: Box<dyn Bird>) {
    arg.fly();
}

pub fn ch23_0_static_dynamic_dispatch() {
    println!("Hello, world!");
    let duck = Duck{};
    test_dispatch_static(duck);
    let swan = Swan{};
    test_dispatch_dyn(Box::new(swan));
}
}
//ch23_1_trait_object();
pub mod ch23_1 {

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) { println!("duck duck"); }
}

impl Bird for Swan{
    fn fly(&self) { println!("swan swan"); }
}
// https://cheats.rs/#references-pointers-ui
fn print_traitobject(p: &dyn Bird) {
    let (data, vtable): (usize, * const usize) = unsafe { std::mem::transmute(p) };
    println!("TraitObject\t[data: {}, vtable: {:p}]", data, vtable);
    unsafe {
        println!("Data in vtable\t[{}, {}, {}, {}]",
            *vtable, *vtable.offset(1), *vtable.offset(2), *vtable.offset(3));
    }
}
pub fn ch23_1_trait_object() {
    println!("Hello, world!");
    let duck = Duck;
    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    println!("Size of p_duck {}, Size of p_bird {}", std::mem::size_of_val(&p_duck), std::mem::size_of_val(&p_bird));
    let duck_fly: usize = Duck::fly as usize;
    let swan_fly: usize = Duck::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);
    print_traitobject(p_bird);
    let swan = Swan;
    print_traitobject(&swan as &dyn Bird);
}
}
