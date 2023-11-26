const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else { 
        speed
    }
}

fn main() {
    println!("{:?}", clamp_speed(9000));
    println!("MAX_SPEED: {}, output: {:?}", MAX_SPEED, clamp_speed(9001));
}
