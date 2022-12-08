// filter_map
//fn filter_map<B, F>(self, f: F) -> impl Iterator<Item=B>
//    where self: Sized, F: FnMut(Self::Item) -> Option<B>;

use std::str::FromStr;

fn main() {
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| { 
            print!("move in value: {:?}\t", w); 
            let ret_val = f64::from_str(w).ok();
            println!("mapped result: {:?}", ret_val); 
            ret_val
        })
    {   println!("for loop handle (sqrt) result: {:4.2}", number.sqrt()); }
dbg!("---- end ----");

    // combine map + filter + map
    for number in text
        .split_whitespace()
        .map(|w| { 
            print!("move in value: {:?}\t", w);
            let mapped_val = f64::from_str(w);
            println!("mapped result: {:?}", mapped_val);
            mapped_val
        })
        .filter(|r| {
            let check_ok = r.is_ok();
            println!("check_ok: {:?}", check_ok);
            check_ok
        })
        .map(|r| {
            let unwrapped_result = r.unwrap();
            println!("unwrapped_result: {unwrapped_result:?}");
            unwrapped_result
        })
    {   println!("for loop handle (sqrt) result: {:4.2}", number.sqrt()); }
dbg!("---- end ----");

    // flat_map
    //fn flat_map<U, F>(self, f: F) -> impl Iterator<Item=U::Item>
    //   where F: FnMut(Self::Item) -> U, U: IntoIterator;
    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]); 
    major_cities.insert("The United States", vec!["Portland", "Nashville"]); 
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]); 
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]); 
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}
