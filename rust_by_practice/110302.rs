use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    //let teams_map2: HashMap<_,_> = teams.into_iter().collect();
    let teams_map2: HashMap<_, _> = IntoIterator::into_iter(teams).collect();
    //let teams_map2 = HashMap::from(teams);
    assert_eq!(teams_map1, teams_map2);

    println!("Ok");
}
