#![feature(generators, generator_trait)]

use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
/*
fn main() {
    let mut gen = || {
        yield 1;
        yield 2;
        ()
    };

    loop {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {}", y),
            GeneratorState::Complete(c) => {
                println!("Complete: {:?}", c);
                break;
            }
        }
    }
}
*/

fn main() {
    let mut gen = Gen::new();

    loop {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {}", y),
            GeneratorState::Complete(c) => {
                println!("Complete: {:?}", c);
                break;
            }
        }
    }
}

enum Gen {
    Enter,
    State1(State1),
    State2(State2),
    Exit
}

struct State1 {
    _x: i32
}

struct State2 {
    _x: i32
}

impl<R> Generator<R> for Gen {
    type Yield = i32;
    type Return = ();

    fn resume(self: Pin<&mut Self>, _arg: R) -> GeneratorState<Self::Yield, Self::Return> {
        let mut_gen = self.get_mut();
        match std::mem::replace(mut_gen, Gen::Exit) {
            Gen::Enter => {
                // do works
                *mut_gen = Gen::State1(State1 { _x: 1 });
                GeneratorState::Yielded(1)
            }
            Gen::State1(_) => {
                // do works
                *mut_gen = Gen::State2(State2 { _x: 2 });
                GeneratorState::Yielded(2)
            }
            Gen::State2(_) => {
                // do works
                *mut_gen = Gen::Exit;
                GeneratorState::Complete(())
            }
            Gen::Exit => panic!("Generator has been completed.")
        }
    }
}

impl Gen {
    fn new() -> Self {
        Self::Enter
    }
}