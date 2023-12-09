#[derive(Debug)]
struct Odd {
    number: isize,
    max: isize,
}
impl Odd {
    fn new(max: isize) -> Self {
        Self { number: -1, max }
    }
}
impl Iterator for Odd {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number <= self.max {
            Some(self.number)
        } else { None }
    }
}

fn main() {
    let mut odds = Odd::new(7);
    println!("{:?}", odds.next());
    println!("{:?}", odds.next());
    println!("{:?}", odds.next());
    println!("{:?}", odds.next());
    println!("{:?}", odds.next());

    let mut odds = Odd::new(7);
    for o in odds {
        println!("odd: {o}");
    }

    let mut evens = Odd::new(8);
    for e in evens.map(|odd| odd + 1) {
        println!("even: {e}");
    }
}
