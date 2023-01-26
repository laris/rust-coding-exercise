fn main() {
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {count}");
    };

    inc();

    let _reborrow = &count;
    //inc();

    let count_reborrowed = &mut count;
    assert_eq!(count, 1);
}
/*
/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 

    assert_eq!(count, 0);
}

*/
