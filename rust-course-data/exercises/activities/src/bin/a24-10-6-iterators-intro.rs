fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num + 1);
    }
    println!("for loop return Vec plus_one: {:?}", plus_one);

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();
    println!("iter+map+collect : {:?}", plus_one);

    let nums_filter: Vec<_> = numbers
        .iter()
        //.filter(|num| **num <= 3) // option 1
        .filter(|&&num| num <= 3)   // option 2
        .collect();
    println!("iter+filter+collect: {:?}", nums_filter);

    let nums_find: Option<&i32> = numbers
        .iter()
        //.find(|num| **num >= 3);
        .find(|&&num| num >= 3);
    println!("iter+find: {:?}", nums_find);

    let count = numbers
        .iter()
        .count();
    println!("iter+count: {:?}", count);

    let last = numbers
        .iter()
        .last();
    println!("iter+last: {:?}", last);

    let numbers: Vec<i32> = vec![];
    let last = numbers
        .iter()
        .last();
    println!("iter+last with empty vec: {:?}", last);

    let numbers: Vec<_> = vec![1, 2, 3, 4, 5, 6];
    let min = numbers
        .iter()
        .min();
    println!("iter+min, same max: {:?}", min);

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let take: Vec<_> = numbers
        .iter()
        .take(3)
        .collect();
    println!("iter+take+collect: {:?}", take);
}
