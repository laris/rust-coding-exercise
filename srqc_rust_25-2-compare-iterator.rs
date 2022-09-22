// Fibonacci with iterator

struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        // confirm overflow
        let new_next = self.curr.checked_add(self.next);
        if let Some(new_next) = new_next {
            // update internal status then return
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        } else {
            // overflow, stop iterate
            None
        }
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    println!("Hello, world!");
    let mut it = fibonacci();
    while let Some(i) = it.next() {
        println!("{}", i);
    }
    println!("u64 max:\n{}", u64::MAX);
}
