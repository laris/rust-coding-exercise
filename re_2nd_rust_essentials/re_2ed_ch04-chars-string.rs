fn main() {
  let magician = "Merlin";
  // does not compile:
	// error: the trait `core::ops::Index<_>` is not implemented 
	// for the type `collections::string::String` [E0277]
	// in Unicode: each character can be a variable number of bytes
  // println!("{}", (magician.to_string())[0]);
 
  let mut str = String::new();
  str.push_str("Gandalf");
 
  // does not compile:
	// error: the trait `core::ops::Index<_>` is not implemented 
	// for the type `collections::string::String` [E0277]
	// in Unicode: each character can be a variable number of bytes
	// println!("{}", str[3]);

  // solution: use a iterator:
  let greeting = "Hello, ä¸–ç•Œ!";
  println!("Bytes:");
  for c in greeting.bytes() { // same as as_bytes()
    print!("{} - ", c);
  }
  println!("");
  println!("Chars:");
  for c in greeting.chars() {
    print!("{} - ", c);
  }
  println!("");
}

// Bytes:
// 72 - 101 - 108 - 108 - 111 - 44 - 32 - 228 - 184 - 150 - 231 - 149 - 140 - 33 - 
// Chars:
// H - e - l - l - o - , -   - ä¸– - ç•Œ - ! - 
