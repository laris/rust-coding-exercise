fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            //todo
        },
        _ => {
            // todo
        },
    };
    never_return_fn()
}

fn never_return_fn() -> ! {
    unimplemented!();
    todo!();
    panic!("panic");
    loop{
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
