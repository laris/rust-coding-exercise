use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
struct User {
    id: i32,
    name: String,
}

// demo 2
#[derive(Debug, PartialEq)]
struct User2 {
    id: i32,
    name: String,
}
impl PartialOrd for User2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.name < other.name {
            Some(Ordering::Less)
        } else if self.name > other.name {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

// demo 3
#[derive(Debug, PartialEq)]
struct User3 {
    id: i32,
    name: String,
}
impl PartialOrd for User3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}
fn main() {
    let a = User { id: 1, name: "a".to_owned() };
    let b = User { id: 2, name: "b".to_owned() };
    if a == b {
        println!("same: {a:?}, {b:?}");
    } else if a < b {
        println!("{a:?} < {b:?}");
    }
    // demo2
    let a = User2 { id: 1, name: "z".to_owned() };
    let b = User2 { id: 2, name: "b".to_owned() };
    if a == b {
        println!("same: {a:?}, {b:?}");
    } else if a < b {
        println!("{a:?} < {b:?}");
    } else if a > b {
        println!("{a:?} > {b:?}");
    }
    // demo3
    let a = User3 { id: 1, name: "z".to_owned() };
    let b = User3 { id: 2, name: "b".to_owned() };
    if a == b {
        println!("same: {a:?}, {b:?}");
    } else if a < b {
        println!("{a:?} < {b:?}");
    } else if a > b {
        println!("{a:?} > {b:?}");
    }
}
