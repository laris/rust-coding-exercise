/*
[RUST 所有权移动问题 - V2EX](https://v2ex.com/t/978382)
https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=0332a1916813a992d3c6549fcebc2b53
qdwang @comment 3
rust 编译过程中，除非是特殊指定在编译器运算的代码（比如 const fn ），其他代码都是不会运算的，只能根据一些特殊情况做一些处理。比如 loop 内非条件语句下认定为会执行到。for 里不一定必然执行。还有例如有些循环情况算必然执行多次，这样就不可以用 FnOnce 这样。
*/
#![allow(unused_mut)]
struct Dog {
    name: String,
}

impl Dog {
    fn release(self) {
        println!("{}", self.name)
    }
}

struct Person {
    dog: Dog,
    name: String,
}

fn main() {
    let mut person = Person {
        dog: Dog {
            name: String::from("Hamel"),
        },
        name: String::from("Zoe"),
    };

    // drop item dog
    person.dog.release();
            

    // try to add item dog back to struct
    // direct way, works
    //person.dog = Dog { name: String::from("Hogge"), };

    // try to add item dog back to struct
    // for method, will report moved
    /*
    for _ in 0..1 {
        person.dog = Dog {
            name: String::from("Hogge"),
        };
    }
    */

    // try to add item dog back to struct
    // loop with if true with single break, failed like for
    /*
    loop {
        if true {
            person.dog = Dog { name: String::from("Hogge"), };
        }
        break
    }
    */

    // try to add item dog back to struct
    // loop with if true with break statement, works
    /*
    */
    loop {
        if true {
            person.dog = Dog { name: String::from("Hogge"), };
            break;
        }
    }

    // try to add item dog back to struct
    // loop method wo if true, pass
    /*
    loop {
        person.dog = Dog { name: String::from("Hogge"), };
        break
    }
    */

    // try to use struct again
    println!("{} got a new dog: {}", person.name, person.dog.name);
}
