use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
	//let f = File::open("hello.txt")?;
	println!("{:?}", File::open("hello.txt")?);
	Ok(())
}