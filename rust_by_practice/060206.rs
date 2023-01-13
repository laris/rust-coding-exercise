fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    let name0 = names.get(0).unwrap();
    let name1 = names.get(1).unwrap();
    let name2 = names.get(2);

    println!("{name0}, {name1}, {name2:?}");
    //let _name1 = &names[2]; // 0, 1
    let _name1 = &names[1]; // 0, 1
}
