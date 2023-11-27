use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(num: num::ParseIntError) -> Self {
        CliError::ParseError(num)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents = fs::read_to_string(&file_name)?;

    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {

    match open_and_parse_file("file") {
        Ok(f) => println!("Ok({f:?})"),
        Err(e) => println!("Err({e:?})"),
    }
    println!("Ok");
}
