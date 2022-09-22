// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);

    // 
    use std::fmt::Debug;
    fn print_it( input: impl Debug + 'static )
    {
        println!( "'static value passed in is: {:?}", input );
    }

    fn use_it()
    {
        // i is owned and contains no references, thus it's 'static:
        let i = 5;
        print_it(i);
        // oops, &i only has the lifetime defined by the scope of
        // use_it(), so it's not 'static:
        //print_it(&i);
    }

    use_it();
}
