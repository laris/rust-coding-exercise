use std::net::{UdpSocket, ToSocketAddrs};
use std::thread;
use std::{io, str};
// ./bili_danhuang_rust_prj_12_udp_chat 127.0.0.1:9520 127.0.0.1:9530 Ann
// ./bili_danhuang_rust_prj_12_udp_chat 127.0.0.1:9530 127.0.0.1:9520 Bob
fn main() {
    println!("UDP Chat App");
    // fn_create("127.0.0.1:9520", "127.0.0.1:9530");
    let mut cnt = 0;
    let (mut src, mut dst) = (None, None);
    let mut name = String::from("user_default");
    for arg in std::env::args() {
        if 1 == cnt { src = Some(arg);}
        else if 2 == cnt { dst = Some(arg); }
        else if 3 == cnt { name = String::from(arg); }
        cnt += 1;
    }
    if let (Some(s), Some(d)) = (src, dst) { fn_create(s, d, name); }
}

fn fn_create<A: ToSocketAddrs>(src: A, dst: A, name: String) {
    println!("[fn_create] Create a chat app");

    let socket = UdpSocket::bind(src);
    if socket.is_err() { return; }

    let socket = socket.unwrap();
    if socket.connect(dst).is_err() { return; }

    let lis_tk = socket.try_clone().unwrap(); let n1 = name.clone();
    let rep_tk = socket.try_clone().unwrap(); let n2 = name.clone();

    let handle_lis = thread::spawn(move || { fn_lis(lis_tk, n1) } );
    let handle_rep = thread::spawn(move || { fn_rep(rep_tk, n2) } );
    handle_lis.join();
    handle_rep.join();

}

fn fn_lis(socket: UdpSocket, name: String) {
    loop {
        let mut buf = [0u8; 256];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let buf = &mut buf[..amt];
        let mut info = String::new();
        for v in buf {
           if '\n' == (*v as char) || '\r' == (*v as char) { continue; }
           info.push(*v as char);
        }
        // println!("[{name}] recv info: {info}");
        println!("{info}");
    }
}

fn fn_rep(socket: UdpSocket, name: String) {
    loop {
        let mut input = String::new(); // reply msg from stdin
        io::stdin().read_line(&mut input).unwrap(); // read stdin
        let msg = format!("[{name}]: {input}"); // prefix name
        if socket.send(msg.as_bytes()).is_err() { continue; } // send msg and check err
        // println!("[{name}] send info: {msg}"); // show
    }
}