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

	if req_line == "GET / HTTP/1.1" {
		let status_line = "HTTP/1.1 200 OK";
		let contents = fs::read_to_string("hello.html").unwrap();
		let len = contents.len();
		let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
		stream.write_all(resp.as_bytes()).unwrap();
	} else {
		// other req
	}

	// println!("Request: {:#?}", http_request);
	// let response = "HTTP/1.1 200 OK\r\n\r\n";
	// stream.write_all(response.as_bytes()).unwrap();
}