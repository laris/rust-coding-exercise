fn example1() {
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}

fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}

fn main() {
    example1();
    example2();

    println!("Success!")
}
