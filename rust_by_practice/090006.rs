#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    pub fn color(&self) -> &str {
        // or -> String
        match *self {
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Green => "green",
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{c:?}");
}
