
/// A last-in, first-out LIFO queue of any type
pub struct Queue<T> {
    older: Vec<T>, // older elements, eldest last.
    younger: Vec<T> // younger elements, youngest last.
}

impl<T> Queue<T> {
    //pub fn new() -> Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // Now older is guaranteed to have something. 
        // Vec's pop method already return an Option, so we're set.
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

fn main() {
  let mut p = Queue::<char>::new();
  p.push('x');

  let mut q = Queue::new();
  let mut r = Queue::new();

  q.push("CAD");  // apparently a Queue<&'static str>
  r.push(0.74);   // apprarently a Queue<f64>
  q.push("BTC");  // Bitcoins per USD, 2017-5
  r.push(2737.7); // Rust fails to detect irrational exuberance

}

