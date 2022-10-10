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
	let http_request: Vec<_> = buf_reader
			.lines()
			.map(|result| result.unwrap())
			.take_while(|line| !line.is_empty())
			.collect();
	println!("Request: {:#?}", http_request); // get req

	// let response = "HTTP/1.1 200 OK\r\n\r\n";
	// construct resp
	let status_line = "HTTP/1.1 200 OK";
	let contents = fs::read_to_string("hello.html").unwrap();
	let len = contents.len();
	// finish full response
	let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{contents}");
	stream.write_all(resp.as_bytes()).unwrap();
}