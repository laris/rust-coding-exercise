fn foo<const N: usize>() {}
fn bar<T, const M: usize>() {
    foo::<M>();
    foo::<2022>();
    foo::<{20 * 100 + 20 + 10 + 1 }>();
    foo::<{ M +1 }>(); // error
    foo::<{ std::mem::size_of::<T>() }>(); // err

    let _: [u8; M]; // ok
    let _: [u8; std::mem::size_of<T>()]; //err
}

fn main() {}
