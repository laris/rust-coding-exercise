#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    println!("Hello, world!");
    let mut g = || {
        yield 1_i32;
        yield 2_i32;
        yield 3_i32;
        return 4_i32;
    };
    loop {
        match Pin::new(&mut g).resume(()) {
            GeneratorState::Yielded(v) => println!("{}", v),
            GeneratorState::Complete(v) => { println!("{}", v); return} ,
        }
    }
}
