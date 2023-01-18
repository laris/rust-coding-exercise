struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

fn main() {
    let mut light = TrafficLight { color: "red".to_string(), };
    light.show_state();
    light.change_state();
    light.show_state();
}
