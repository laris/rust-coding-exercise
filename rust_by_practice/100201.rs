struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}
use std::fmt::Debug;

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
}

fn main() {}
