extern crate protocol;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io;

fn main() {
    println!("What port should the application listen on?");
    let mut port = String::new();
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to get port");

    println!("Master information in IP:Port")
    let mut master = String::new();
    io::stdin()
        .read_line(&mut master)
        .expect("Failed to read");

    let listener = TcpListener::bind("0.0.0.0:{}", port);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

    }
}
