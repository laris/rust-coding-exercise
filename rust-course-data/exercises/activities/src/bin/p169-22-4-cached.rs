//# cached = "*"
use cached::proc_macro::cached;

//#[cached]
//#[cached(size = 10)]
#[cached(size = 50, time = 30)]
fn expensive(n: usize) -> usize {
    thread::sleep(Duration::from_millis(500));
    match n {
        1 => 1,
        2 => 2,
        _ => n,
    }
}

#[cached(option = true)]
fn expensive_1(n: usize) -> Option<usize> {}

#[cached(option = true)]
fn expensive_2(n: usize) -> Result<usize, String> {}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Choice {
    A,
    B,
    C,
}
#[cached]
fn expensive_3(choice: Choice) -> usize {}

fn main() {
    
}
