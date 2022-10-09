use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time;
use std::thread;

fn main() {
	// start first thread for server
	let handle_server = thread::spawn(move || {
		println!("[Server]: Start ...");
		worker_server(); // worker
		println!("[Server]: Close ...");
	});
	// delay 200ms and start 2nd thread for client
	let handle_client = thread::spawn(move || {
		thread::sleep(time::Duration::from_millis(200));
		println!("[Client]: Start ...");
		worker_client(); // woker
		println!("[Client]: Close ...");
	});
	// join svr clt handle
	handle_server.join();
	handle_client.join();
}

fn worker_server() {
	// bind to ip:port
	let listener = TcpListener::bind("127.0.0.1:9528");
	// check bind status
	if !listener.is_ok() {
		println!("[Server]: Bind ip and port fail ...");
		return;
	}
	// unwrap
	let listener = listener.unwrap();
	println!("[Server]: Waiting for next msg ...");
	// iter mesg
	for stream in listener.incoming() {
		// check error
		if !stream.is_ok() { println!("[Server]: Getting error msg..."); continue; }
		// unwrap
		let mut stream = stream.unwrap();
		// process stream
		process_stream(stream);
		// end loop
		println!("[Server]: Waiting for the next msg...");
	}
}
// process stream
fn process_stream(mut stream: TcpStream) -> bool {
	let mut buf = [0; 1024];  // buf
	// read req from TcpStream
	if !stream.read(&mut buf).is_ok() { return false; } // read and check err
	// ok now and check buf
	println!("[Server][fn process_stream] Get req info: {}", String::from_utf8_lossy(&buf[..]));
	// resp to stream and check write status if exist err
	println!("[Server] Send Response to client: (Server->Client) Server has recved ur req ...");
	if !stream.write(b"(Server->Client) Server has recved ur req ...").is_ok() { return false; }
	return true;
}

fn worker_client() {
	let mut stream = TcpStream::connect("127.0.0.1:9528");
	// check connect err
	if !stream.is_ok() { println!("[Client]: Bind failed ..."); return; }
	let mut stream = stream.unwrap();
	// send request msg to server
	println!("[Client]: Send Request msg to server: (Client->Server) Client send request info to server!");
	let req_status = stream.write(b"(Client->Server) Client send request info to server!");
	// check send status
	if !req_status.is_ok() { println!("[Client]: Send info fail ..."); return; }

	let mut buf = [0; 1024]; // buf
	let response = stream.read(&mut buf); // get resp
	if !response.is_ok() { println!("[Client]: Recv resp info fail ..."); return; }
	// check server's resp msg
	println!("[Client]: Get resp msg from server {}", String::from_utf8_lossy(&buf[..]));
	stream.shutdown(Shutdown::Both); // shutdown stream
}