fn main() {
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);

    fn function(i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32|  -> i32 { i + 1 };
    let closure_inferred  = |i|                i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    let one = || 1;
    println!("closure returning one: {}", one());

//--------------------------------------------------------------------------------
    // Increment via closures and functions.
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // 
    // These nameless functions are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

}
