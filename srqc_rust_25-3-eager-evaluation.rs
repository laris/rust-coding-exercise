// Fibonacci eager evaluation

fn collector() -> Vec<u64> {
    let mut res = vec![];
    let mut curr: u64 = 1;
    let mut next: u64 = 1;
    loop {
        let new_next = curr.checked_add(next);
        if let Some(new_next) = new_next {
            curr = next;
            next = new_next;
            res.push(curr);
        } else {
            break;
        }
    }
    return  res;
}
fn main() {
    println!("Hello, world!");
    let collected = collector();
    let mut it = collected.iter();
    while let Some(i) = it.next() {
        println!("{}", i);
    }
    println!("u64 max:\n{}", u64::MAX);
}
