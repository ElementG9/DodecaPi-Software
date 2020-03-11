use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub bytes: Vec<u8>,
}
impl Packet {
    pub fn get_command_string(&self) -> String {
        String::from_utf8_lossy(&self.bytes).to_string()
    }
}

pub fn start_listener(port: u16) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(format!("127.0.0.1:{}", port))
}
pub fn handle_stream(mut stream: &mut TcpStream, conf: &crate::config::Config) {
    let start = read_le_u32(&mut stream).unwrap();
    let end = read_le_u32(&mut stream).unwrap();
    let command = read_packet(&mut stream).unwrap();
    println!("Range: {} to {}", start, end);
    println!("Command: {}", command.get_command_string());
    let mut cluster_streams = Vec::new();
    for cluster_client in &conf.cluster {
        let address = format!("{}:{}", cluster_client, conf.portout);
        cluster_streams.push(
            connect_client(&address)
            .expect(&format!("Could not connect client {}", &address))
        );
    }
}
pub fn connect_client(address: &String) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(&address)
}
// Helper functions.
pub fn read_packet(mut stream: &mut TcpStream) -> Result<Packet, std::io::Error> {
    let length = read_le_u32(&mut stream)?;
    let mut bytes = Vec::new();
    for _ in 0..length {
        bytes.push(read_byte(&mut stream)?);
    }
    Ok(Packet {
        length,
        bytes,
    })
}
pub fn read_byte(mut stream: &mut TcpStream) -> Result<u8, std::io::Error> {
    let mut buffer: [u8; 1] = [0; 1];
    stream.read(&mut buffer)?;
    Ok(buffer[0])
}
pub fn read_le_u32(mut stream: &mut TcpStream) -> Result<u32, std::io::Error> {
    let mut buffer: [u8; 4] = [0; 4];
    stream.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}
pub fn write_le_u32(mut stream: &mut TcpStream, num: u32) -> Result<(), std::io::Error> {
    stream.write(&num.to_le_bytes())?;
    Ok(())
}
