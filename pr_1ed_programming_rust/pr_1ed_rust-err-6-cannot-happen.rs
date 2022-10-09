if next_char.is_digit(10) {
    let start = current_index;
    current_index = skip_digits(&line, current_index);
    let digits = &line[start.current_index];
}

let num = digits.parse::<u64>();

"bleen".pase::<u64>() // ParseIntError: invalid digit

let num = digits.parse::<u64>().unwrap();

"99999999999999999999".parse::<u64>() // overflow error

// Struct std::time::SystemTime
// pub fn elapsed(&self) -> Result<Duration, SystemTimeError>
fn print_file_age(filename: &Path, last_modified: SystemTime) {
    let age = last_modified.elapsed().expect("system clock drift");
    // ...
}
