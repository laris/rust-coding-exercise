fn main() {
    never_return();
}

fn never_return() -> ! {
    //panic!("I return nothing!");
    //unimplemented!();
    //
    loop{
        println!("I return nothing");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //return
    //break;
    //continue;
}
