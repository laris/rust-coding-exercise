fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    let a_is_some = a.is_some();
    println!("a_is_some: {:?}", a_is_some);
    
    let a_is_none = a.is_none();
    println!("a_is_none: {:?}", a_is_none);
    
    let a_mapped =  a.map(|num| num + 1);
    println!("a_mapped: {:?}", a_mapped);
    
    let a_filtered = a.filter(|num| num == &1);
    println!("a_filtered: {:?}", a_filtered);

    let a_filtered_change = a.filter(|num| num == &1).map(|num| num + 1);
    println!("a_filtered_change: {:?}", a_filtered_change);

    let a_or_else = a.or_else(|| Some(5));
    println!("a_or_else: {:?}", a_or_else);
    let b: Option<i32> = None;
    let b_or_else = b.or_else(|| Some(5));
    println!("b_or_else: {:?}", b_or_else);

    let a_unwrapped = a.unwrap_or_else(|| 0);
    println!("a_unwrapped: {:?}", a_unwrapped);
    let b_unwrapped = b.unwrap_or_else(|| 0);
    println!("b_unwrapped: {:?}", b_unwrapped);
    dbg!(b_unwrapped);
}
