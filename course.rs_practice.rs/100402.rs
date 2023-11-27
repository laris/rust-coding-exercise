struct Duck;
impl Duck {
    fn fly(&self) { println!("Look, the duck is flying");}
}

struct Swan;
impl Swan {
    fn fly(&self) { println!("Look, the duck.. oh sorry, the swan is flying");}
}

trait Bird {
    fn quack(&self);
}

impl Bird for Duck {
    fn quack(&self) { println!("{}", "duck duck");}
}
impl Bird for Swan{
    fn quack(&self) { println!("{}", "swan swan");}
}

fn main() {
    let birds: [Box<dyn Bird>;2] = [ Box::new(Duck{}), Box::new(Swan{}) ];
    for bird in birds {
        bird.quack();
    }
}
