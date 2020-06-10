extern crate protocol;
use protocol::packet::*;
use std::net::TcpStream;

static SERVER_ADDRESS: &str = "0.0.0.0";
static SERVER_PORT: u16 = 6500;
static CLIENT_PROTOCOL_VERSION: u8 = 2;

fn main() {
    let mut stream = TcpStream::connect(&format!("{}:{}", SERVER_ADDRESS, SERVER_PORT))
        .expect("could not connect to server");
    let _ = Handshake::new(CLIENT_PROTOCOL_VERSION).write(&mut stream);
    let response = HandshakeResponse::read(&mut stream);
}
