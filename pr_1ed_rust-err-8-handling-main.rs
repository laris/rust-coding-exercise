fn main() {
    calculate_tides()?; // error: can't pass the buck any further
    calculate_tides().expect("error"); // the buck stops here

    if let Err(err) = calculate_tides() {
        print_error(&err);
        std::process::exit(1);
    }
}

