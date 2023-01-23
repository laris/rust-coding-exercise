fn main() {
    // From/Into demo
    let my_str = "hello";
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    let string3: String = my_str.into();

    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    //impl From<char> for i32 {}
    //let i3: i32 = 'a'.into();
    //let s: String = 'a' as String;

    let i3: u32 = 'a'.into();
    let s: String = 'a'.into();

    let i3: u32 = 'a' as u32;
    let s: String = String::from('a');
}
