extern crate protocol;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io;

fn main() {
    println!("Which interface should the application listen on? (Example: 0.0.0.0:420 for all interfaces, port 420)");
    let mut listen = String::new();
    io::stdin()
        .read_line(&mut listen)
        .expect("Failed to read");

    println!("Master information in IP:Port");
    let mut master = String::new();
    io::stdin()
        .read_line(&mut master)
        .expect("Failed to read");

    let listener = TcpListener::bind(listen).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

    }
}
