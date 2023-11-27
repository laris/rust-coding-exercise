struct Cacher<T, E>
    where
    T: Fn(E) -> E,
    E: Copy
{
        query: T,
        value: Option<E>,
}

impl<T,E> Cacher<T,E>
    where
    T: Fn(E) -> E,
    E: Copy
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query,
            value: None,
        }
    }
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {

}

#[test]
fn call_with_diff_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 1);
}
