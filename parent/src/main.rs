extern crate protocol;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut p = protocol::Packet::new(0x00);
    p.write_string("test".to_owned());

    let bytes = p.get_bytes();
    println!("{:X?}", bytes);

    let length = protocol::Packet::read_u32(0, &bytes);
    let id = protocol::Packet::read_u8(4, &bytes);
    let s = protocol::Packet::read_string(5, &bytes);
    println!("Length: {}, ID: {}, String: \"{}\"", length, id, s);

    let mut stream = TcpStream::connect("127.0.0.1:6500").unwrap();

    stream.write(&bytes);
}
