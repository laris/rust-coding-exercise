fn main() {
    let upper_case: String = "große".chars()
        .inspect(|c| print!("before: {:?}\t", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");
    println!("----");
    // more
    // failed, because println already output the stream, so cannot print backspace
    // and return to previous line
/*
    use std::cell::RefCell;
    let counter = RefCell::new(0);
    let upper_case: String = "große".chars()
        .inspect(|c| {
            *counter.borrow_mut() = 0;
            print!("before: {:?}, counter: {}\t", c, counter.borrow())
        })
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| {
            if *counter.borrow() == 0 {
                println!("after: {:?}, counter: {}\t", c, counter.borrow())
            } else if *counter.borrow() == 1 {
                println!("\x08after: {:?}, counter: {}\t", c, counter.borrow())
            }
            *counter.borrow_mut() += 1;
        })
        .collect();
    assert_eq!(upper_case, "GROSSE");
    println!("----");
    // try another solution, ToUppercase enum, failed because private struct
    //use std::char::ToUppercase;
    // try check mem size, failed
    //use std::mem::*;
    let upper_case: String = "große".chars()
        .inspect(|c| {
            println!("after: {:?}", c)
        })
        .flat_map(|c| {
            let ret = c.to_uppercase();
            println!("return: {:?}", ret.size_hit());
            ret
        })
        .inspect(|c| {
            println!("after: {:?}", c)
        })
        .collect();
    assert_eq!(upper_case, "GROSSE");
    println!("----");
*/
}
