extern crate protocol;
use protocol::packet::*;
use std::net::{TcpListener, TcpStream};

static SERVER_PORT: u16 = 6500;
static SERVER_PROTOCOL_VERSION: u8 = 3;

fn main() {
    let listener_address = &format!("0.0.0.0:{}", SERVER_PORT);
    let listener = TcpListener::bind(listener_address).expect("could not create listener");
    println!("Started listener at {}", listener_address);
    for stream in listener.incoming() {
        handle_connection(Connection {
            state: ConnectionState::Handshake,
            stream: stream.expect("could not make connection"),
        });
    }
}
enum ConnectionState {
    Handshake,
    Ping,
    Work,
    Wait,
    Disconnected,
}
struct Connection {
    pub state: ConnectionState,
    pub stream: TcpStream,
}
fn handle_connection(mut c: Connection) -> std::io::Result<()> {
    println!("Got a connection");
    loop {
        match c.state {
            ConnectionState::Handshake => {
                // Read 0x00 Handshake
                let (_packet_len, packet_id) = read_packet_header(&mut c.stream)?;
                if packet_id != 0x00 {
                    // If not 0x00 Handshake
                    println!("Didn't get 0x00 Handshake when expected. Disconnecting...");
                    disconnect(&mut c)?;
                    return Ok(());
                }
                let p = Handshake::read(&mut c.stream)?;
                if p.protocol_version != SERVER_PROTOCOL_VERSION {
                    // Protocol versions don't match
                    println!("Protocol versions don't match. Disconnecting...");
                    disconnect(&mut c)?;
                    return Ok(());
                }
                c.state = match p.next_state {
                    0x00 => ConnectionState::Ping,
                    0x01 => ConnectionState::Work,
                    0x02 => ConnectionState::Wait,
                    _ => {
                        // Unexpected next state
                        println!("Unexpected next state. Disconnecting...");
                        disconnect(&mut c)?;
                        ConnectionState::Disconnected
                    }
                };
                println!("Got 0x00 Handshake");
                // Write 0x08 Handshake Response
                println!("Sending 0x08 Handshake Response");
                let _ = HandshakeResponse::new().write(&mut c.stream)?;
            }
            ConnectionState::Ping => {
                // Read 0x09 Ping
                let (_packet_len, packet_id) = read_packet_header(&mut c.stream)?;
                if packet_id != 0x09 {
                    // If not 0x09 Ping
                    println!("Didn't get 0x09 Ping when expected. Disconnecting...");
                    disconnect(&mut c)?;
                    return Ok(());
                }
                let _ = Ping::read(&mut c.stream)?;
                println!("Got 0x09 Ping");
                // Write 0x10 Pong
                println!("Sending 0x10 Pong");
                let _ = Pong::new().write(&mut c.stream)?;
            }
            ConnectionState::Work => {
                // Do some work here and return result
                // Return to the wait state for further instructions
                ConnectionState::Wait;
            }
            ConnectionState::Wait => {
                println!("Awaiting instructions");
                let (_packet_len, packet_id) = read_packet_header(&mut c.stream)?;
                if packet_id == 0x07 {
                    println!("Master wanted a disconnect");
                    disconnect(&mut c)?;
                    return Ok(());
                }
            }
            ConnectionState::Disconnected => {}
        }
    }
}
fn disconnect(c: &mut Connection) -> std::io::Result<()> {
    c.state = ConnectionState::Disconnected;
    protocol::packet::Disconnect::new().write(&mut c.stream)?;
    c.stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
