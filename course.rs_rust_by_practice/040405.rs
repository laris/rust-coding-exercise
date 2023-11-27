fn main() {
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Sucess!");
            panic!("We have no value for false, but we can panic");
        }
    };
    println!("Execise failed if printing out this line!");
}
