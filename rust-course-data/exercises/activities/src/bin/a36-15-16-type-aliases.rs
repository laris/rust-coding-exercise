use std::collections::HashMap;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;

fn add_contact(index: &mut ContactIndex, contact: Contact) {
    //index.insert(contact.phone.to_owned(), contact);
    index.insert(contact.name.to_owned(), contact);
}

// Generics/Lifetimes
type BorrowedItems<'a> = Vec<&'a str>;
struct Thing<T>(T);
type GenericThings<T>  = Vec<Thing<T>>;

fn main() {
    let mut contact_index: HashMap<ContactName, Contact> = HashMap::new();
    let contact = Contact { name: "Jimmy".into(), phone: "+86-13800138000".into() };
    add_contact(&mut contact_index, contact);
    println!("{contact_index:?}");
}
