use std::collections::HashMap;

fn main()  {
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);
    assert_eq!(player_stats["health"], 100);

    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Ok");
}

fn random_stat_buff() -> u8 { 42 }
