enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull   => println!("dull"),
    }
}
fn display_light_borrow(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull   => println!("dull"),
    }
}

fn main() {
    let light = Light::Bright;
    display_light(light);
    //display_light(light);
    let light = Light::Dull;
    display_light_borrow(&light);
    display_light_borrow(&light);

}
