#![feature(generators, generator_trait)]

fn main() {
    println!("Hello, world!");
    let g = || {
        let local = 1;
        let ptr = &local; // error[E0626]: borrow may still be in use when generator yields
        yield local;
        yield *ptr;
    };
}
