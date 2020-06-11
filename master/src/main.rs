extern crate protocol;
use protocol::helper::disconnect;
use protocol::packet::*;
use std::net::TcpStream;

static SERVER_ADDRESS: &str = "0.0.0.0";
static SERVER_PORT: u16 = 6500;
static CLIENT_PROTOCOL_VERSION: u8 = 2;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(&format!("{}:{}", SERVER_ADDRESS, SERVER_PORT))
        .expect("could not connect to server");
    // Write 0x00 Handshake
    println!("Sending 0x00 Handshake");
    let _ = Handshake::new(CLIENT_PROTOCOL_VERSION, 0x00).write(&mut stream)?;
    // Read 0x08 Handshake Response
    let (_packet_len, packet_id) = read_packet_header(&mut stream)?;
    if packet_id == 0x07 {
        // Got a 0x07 Disconnect
        println!("Server does not support protocol version. Disconnecting...");
        disconnect(&mut stream)?;
        return Ok(());
    } else if packet_id != 0x08 {
        // If not 0x08 Handshake Response
        println!("Didn't get 0x08 Handshake Response when expected. Disconnecting...");
        disconnect(&mut stream)?;
        return Ok(());
    }
    let _response = HandshakeResponse::read(&mut stream)?;
    println!("Got 0x08 Handshake Response");
    // Write 0x09 Ping
    println!("Sending 0x09 Ping");
    let _ = Ping::new().write(&mut stream)?;
    // Read 0x10 Pong
    let (_packet_len, packet_id) = read_packet_header(&mut stream)?;
    if packet_id != 0x10 {
        // If not 0x08 Handshake Response
        println!("Didn't get 0x10 Pong when expected. Disconnecting...");
        disconnect(&mut stream)?;
        return Ok(());
    }
    let _response = Pong::read(&mut stream)?;
    println!("Got 0x10 Pong");
    Ok(())
}
