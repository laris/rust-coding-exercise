use std::ops;

#[derive(Debug, PartialEq)]
struct Foo;
#[derive(Debug, PartialEq)]
struct Bar;
#[derive(Debug, PartialEq)]
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo{
    type Output = BarFoo;
    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn main() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
}

