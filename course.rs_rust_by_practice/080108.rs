enum Foo {
    Bar, 
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Qux(10);

         if let Foo::Bar = a { println!("match Foo::Bar"); } 
    else if let Foo::Baz = a { println!("match Foo::Baz"); }
    else { println!("match others");}

    match a {
        Foo::Bar => println!("match Foo::Bar"),
        Foo::Baz => println!("match Foo::Baz"),
        _ => println!("match others"),
    }
}
