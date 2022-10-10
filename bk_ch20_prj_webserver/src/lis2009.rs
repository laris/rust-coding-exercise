use std::{
	fs,
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
};

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		// println!("Connection established!");
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&mut stream);
	// println!("{buf_reader:?}");
	let req_line = buf_reader.lines().next().unwrap().unwrap(); // get method
	println!("{req_line:?}");

	// refactor
	let (status_line, filename) =
		if req_line == "GET / HTTP/1.1"
				 { ("HTTP/1.1 200 OK", "hello.html") }
		else { ("HTTP/1.1 404 NOT FOUND", "404.html") };

	let contents = fs::read_to_string(filename).unwrap();
	let len = contents.len();
	let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
	stream.write_all(resp.as_bytes()).unwrap();
}