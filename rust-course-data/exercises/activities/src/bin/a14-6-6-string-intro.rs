#![allow(dead_code)]

fn print_it(data: &str) {
    println!("{:?}", data);
}

#[derive(Debug)]
struct EmployeeStringSlice<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct EmployeeString {
    name: String, 
}

fn main() {
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");

    print_it(&owned_string);
    print_it(&another_owned);

    let emp_name = "Jayson";
    // String Slice with lifetime
    let emp_str = EmployeeStringSlice {
        name: emp_name,
    };
    println!("EmplyeeStringSlice: {:?}", emp_str);
    // String
    let emp_string = EmployeeString {
        //name: emp_name.to_owned(),
        //name: emp_name.to_string(),
        //name: String::from(emp_name),
        name: emp_name.into(),
    };
    println!("EmplyeeStringe: {:?}", emp_string);

}
