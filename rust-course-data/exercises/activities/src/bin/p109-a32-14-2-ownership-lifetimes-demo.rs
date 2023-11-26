#[derive(Debug)]
struct Cards { inner: Vec<IdCard>, }

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City { Barland, Bazopolis, Fooville, }

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self { name: name.to_string(), age, city }
    }
}

fn new_ids() -> Cards {
    Cards { inner: vec![ IdCard::new("Amy", 1, City::Fooville),
            IdCard::new("Matt", 10, City::Barland),
            IdCard::new("Bailee", 20, City::Barland),
            IdCard::new("Anthony", 30, City::Bazopolis),
            IdCard::new("Tina", 40, City::Bazopolis), ], }
}

#[derive(Debug)]
struct YoungPeople<'a> { inner: Vec<&'a IdCard> }

impl<'a> YoungPeople<'a> {
    fn living_in_fooville(&self) -> Self {
        Self { inner: self.inner.iter().filter(|id| id.city == City::Fooville).map(|id| *id).collect(),
        } }
}
fn main() {
    let ids = new_ids();
    let young = YoungPeople { inner: ids.inner.iter().filter(|id| id.age <= 20).collect(), };
    println!("all people");
    for id in ids.inner.iter() { println!("{:?}", id); }
    println!("young people");
    for id in young.inner.iter() { println!("{:?}", id); }
    println!("young people living in Fooville");
    for id in young.living_in_fooville().inner.iter() { println!("{:?}", id); }
}
