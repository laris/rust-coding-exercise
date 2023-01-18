#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    //pub fn new(c: String) -> TrafficLight {
    pub fn new(c: String) -> Self {
        TrafficLight { color: c, }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new("red".to_string());
    assert_eq!(light.get_state(), "red");
}
