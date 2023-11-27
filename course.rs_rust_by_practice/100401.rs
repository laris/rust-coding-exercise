trait Bird {
    fn quack(&self) -> String;
}

struct Duck;

impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) -> String { "duck duck".to_string() }
}

impl Bird for Swan {
    fn quack(&self) -> String { "swan swan".to_string() }
}

fn main() {
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // bird.swim();
    assert_eq!(bird.quack(), "duck duck");
    
    let bird = hatch_a_bird(1);
    // bird.fly();
    assert_eq!(bird.quack(), "swan swan");
    
}

fn hatch_a_bird(n: u8) -> Box<dyn Bird> {
        if n == 1 { Box::new(Swan{}) }
        else { Box::new(Duck{}) }
}
