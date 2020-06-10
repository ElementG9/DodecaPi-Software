extern crate protocol;
use std::net::TcpListener;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Master location (Example: 192.168.35.50:6500)");
    let mut master = String::new();
    io::stdin()
        .read_line(&mut master)
        .expect("Failed to read");

    let mut listener = TcpListener::bind("0.0.0.0:6500").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("got something");
        let mut packetBuffer = Vec::new();
        stream.read_to_end(&mut packetBuffer);
        println!("Hex: {:X?}", packetBuffer);

        let length = protocol::Packet::read_u32(0, &packetBuffer);
        let id = protocol::Packet::read_u8(4, &packetBuffer);
        let s = protocol::Packet::read_string(5, &packetBuffer);
        println!("Length: {}, ID: {}, String: \"{}\"", length, id, s);

        if id == 9 {
            let mut p = protocol::Packet::new(0x00);
            p.write_string("test".to_owned());

            let bytes = p.get_bytes();
            stream.write(&bytes);
        }

    }
}
