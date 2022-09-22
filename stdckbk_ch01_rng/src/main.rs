extern crate rand;

fn main() {
    //println!("Hello, world!");
    let random_num1 = rand::random::<i32>();
    println!("random_num1: {}", random_num1);

    let random_num2: i32 = rand::random();
    println!("random_num2: {}", random_num2);

    let random_char = rand::random::<char>();
    println!("random_char: {}, random_char code: {:#x}", random_char, random_char as u32);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    if rng.gen() {
        println!("This msg has a 50-50 chance of being printed");
    }

    let random_num3 = rng.gen_range(0..=10);
    println!("random_num3: {}", random_num3);

    let random_float = rng.gen_range(0.0..1.0);
    println!("random_float: {}", random_float);

/*
    let mut chacha_rng = rand::ChaChaRng::new_unseeded();
    let random_chacha_num = chacha_rng.gen::<i32>();
    println!("random_chacha_num: {}", random_chacha_num);
*/
}
