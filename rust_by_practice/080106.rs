fn main() {
    let o = Some(7);

    match o {
        Some(i) => { println!("This is a really long string and {:?}", i);}
        _ => {}
    };

    if let Some(i) = o { println!("{i}");}
}
