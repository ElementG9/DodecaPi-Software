extern crate protocol;
use protocol::packet::*;
use std::net::{TcpListener, TcpStream};

static SERVER_PORT: u16 = 6500;
static SERVER_PROTOCOL_VERSION: u8 = 2;

fn main() {
    let listener_address = &format!("0.0.0.0:{}", SERVER_PORT);
    let listener = TcpListener::bind(listener_address).expect("could not create listener");
    println!("Started listener at {}", listener_address);
    for stream in listener.incoming() {
        let _ = handle_connection(stream.expect("could not make connection"));
    }
}
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Got a connection");
    // Read 0x00 Handshake
    let protocol_version = Handshake::read(&mut stream)?.protocol_version;
    println!("Client wants protocol version {}", protocol_version);
    // Reply with 0x08 Handshake Response
    let _ = HandshakeResponse::new().write(&mut stream)?;
    Ok(())
}
