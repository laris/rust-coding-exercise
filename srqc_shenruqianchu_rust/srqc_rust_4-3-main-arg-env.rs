fn main() {
    for arg in std::env::args() {
        println!("arg: {}", arg);
    }

    for arg in std::env::args() {
        match std::env::var(&arg) {
            Ok(val) => println!("{}: {:?}", &arg, val),
            Err(e)  => println!("couldn't find env {}, {}", &arg, e),
        }
        println!("All env variable count {}", std::env::vars().count());
    }
    std::process::exit(0);
    //std::process::exit(1);
}
