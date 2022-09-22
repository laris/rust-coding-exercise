use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

fn main() {
	println!("{:?}", read_username_from_file());
}