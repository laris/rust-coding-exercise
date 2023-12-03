use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug)]
struct Book {
    signed: Cell<bool>,
}

impl Book {
    fn sign(&self) {
        self.signed.set(true);
    }
    fn signed(&self) -> bool {
        self.signed.get()
    }
}
// demo2
struct Person {
    name: RefCell<String>,
}

fn main() {
    let my_book = Book {
        signed: Cell::new(false),
    };
    println!("signed: {}", my_book.signed());
    my_book.sign();
    println!("signed: {}", my_book.signed());
    // demo2
    let person = Person {
        name: RefCell::new("Amy".into()),
    };
    {
        let person_name = person.name.borrow();
        println!("Person's name is: {person_name}");
    }
    {
        let mut person_name = person.name.borrow_mut();
        *person_name = "Tim".into();
        println!("Person's name is: {person_name}");
    }
    {
        let previous_name = person.name.replace("Tom".into());
        let person_name = person.name.borrow();
        println!("New Person's name is: {person_name}\nPrevious Person's name: {previous_name}");
    }

    println!("-------------");
    let person_name = person.name.borrow();
    println!("Person's name is: {person_name}");
    match person.name.try_borrow() {
        Ok(person_name) => {
            println!("Person's name is: {person_name}");
        },
        Err(e) => println!("{e:?}"),
    };
    match person.name.try_borrow_mut() {
        Ok(mut person_name) => {
            *person_name = "Jimmy".into();
            println!("Person's name is: {person_name}");
        },
        Err(e) => println!("{e:?}"),
    };
}
