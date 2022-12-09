use std::collections::BTreeMap;

fn main() {
    //fn flatten(self) -> impl Iterator<Item=Self::Item::Item>
    //    where Self::Item: IntoIterator;

    // A table mapping cities to their parks: each value is a vector.
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]); 
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build a vector of all parks. `values` gives us an iterator producing 
    // vectors, and then `flatten` produces each vector's elements in turn.
//  let all_parks: Vec<_> = parks.values().flatten().clone().collect();
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    println!("{:#?}", all_parks);
    // ["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park", " Mt. Tabor Park", "Forest Park"]
//    assert_eq!(all_parks, vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park", "Mt. Tabor Park", "Forest Park"]);

        assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );

    // Vec<Option<_>>
    assert_eq!(
    vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
    vec!["day", "one"]
    );

    // Option<Vec<_>>
    assert_eq!(
    Some(vec!["Monday", "Tuesday"])
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
    vec!["Monday", "Tuesday"]
    );

    // Vec<Result<_>>
    assert_eq!(
    vec![Ok("one"), Ok("two"), Err("e"), Ok("four")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
    vec!["one", "two", "four"]
    );

    // to_uppercase
    fn my_to_uppercase1(s: &str) -> String {
        s.chars()
         .map(|c| {
                print!("s.chars: {c:?}\t");
                let ret = char::to_uppercase(c);
                println!("char::to_uppercase(c): {ret:?}");
                ret
            })
         .flatten()
         .collect()
    }
    
    assert_eq!(
        my_to_uppercase1("abctsch-ü-ß-农历新年"),
        "ABCTSCH-Ü-SS-农历新年".to_string()
    );

    fn my_to_uppercase2(s: &str) -> String {
        s.chars()
        .flat_map(char::to_uppercase)
        .collect()
    }
    assert_eq!(
        my_to_uppercase2("abc"),
        "ABC".to_string()
    );

    // to_lower_case, it's special char, so below test failed
    // but at the end of a word, it's ς, not σ:
    fn my_to_lowercase(s: &str) -> String {
        s.chars()
        .flat_map(|c| {
                print!("s.chars: {c:?}\t");
                let ret = char::to_lowercase(c);
                println!("char::to_lowercase(c): {ret:?}");
                ret
            })
        .collect()
    }
    let odysseus = "ὈΔΥΣΣΕΎΣ";
    assert_eq!("ὀδυσσεύσ", my_to_lowercase(odysseus));

    let odysseus2 = "ὈΔΥΣΣΕΎΣ";
    for c in odysseus2.chars() {
        println!("{:?}", c.to_lowercase());
    }
}
