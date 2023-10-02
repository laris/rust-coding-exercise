macro_rules! m{
    ($($s:stmt)*) => {
        $(
            //{ stringify!($s); 1 }
            { println!("{}",stringify!($s)); 1 }
         )<<*
    };
}

fn main() {
    println!(
        "{}{}{}",
        m! {  return  || true },
        m! { (return) || true },
        m! { {return} || true },
        );

    //println!("(return) || false output: {}", (return) || false );
    //println!("(return) || false output: {}", (return) || true );
    println!("f1() output: {}", f2());
}

fn f1() -> &'static &'static bool {
    // Block expression.
    {
        println!("What a silly function.");
    }

    // Reference to reference to true.
    &&true
}

fn f2() -> bool {
    //{ true } && true  // error
    ({ false } && false )
    //({ true }) && true

}

/*

PS C:\Users\Laris\prjs\rust-coding-exercise\rust_quiz> rustc .\quiz01.rs
warning: unnecessary braces around block return value
  --> .\quiz01.rs:23:5
   |
23 |     { true } && true
   |     ^^    ^^
   |
   = note: `#[warn(unused_braces)]` on by default
help: remove these braces
   |
23 -     { true } && true
23 +     true && true
   |

error[E0308]: mismatched types
  --> .\quiz01.rs:23:7
   |
23 |     { true } && true
   |       ^^^^ expected `()`, found `bool`
   |
help: you might have meant to return this value
   |
23 |     { return true; } && true
   |       ++++++     +

error[E0308]: mismatched types
  --> .\quiz01.rs:23:14
   |
22 | fn f() -> bool {
   |           ---- expected `bool` because of return type
23 |     { true } && true
   |              ^^^^^^^ expected `bool`, found `&&bool`
   |
help: parentheses are required to parse this as an expression
   |
23 |     ({ true }) && true
   |     +        +

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.

*/
