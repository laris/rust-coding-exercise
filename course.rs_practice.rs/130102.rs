fn main() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);
    let v = vec![1, 2, 3];
    let ele = v[2];
    let ele = v.get(2).unwrap();

    let v = production_rate_per_hour(2);
    divide(15, 1);
    println!("Success!");
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y);
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 21;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
