//# rand = "*"
use rand::prelude::*;

fn main() {
    let number: u8 = random();
    let yes_no: bool = random();
    
    let mut rng = thread_rng();
    let number = rng.gen_range(0..10);

    let letters = ['a', 'b', 'c'];
    let letter = letters.iter().choose(&mut rng);

    let mut letters = letters;
    letters.shuffle(&mut rng);

    //use rand::prelude::*;
    use rand_pcg::Pcg64;
    use rand_seeder::Seeder;

    let rng = Pcg64::seed_from_u64(10);
    let rng: Pcg64 = Seeder::from("seed value").make_rng();

    //# rand = "*"
    //# rand_distr = "*"
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::*;

    let range = Uniform::from(5..500);
    let mut rng = thread_rng();
    range.sample(&mut rng);
}
