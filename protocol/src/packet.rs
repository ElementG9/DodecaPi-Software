use crate::helper::*;
use std::net::TcpStream;

pub fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(u32, u8)> {
    let packet_length = read_u32(t)? - 1;
    let packet_id = read_u8(t)?;
    Ok((packet_length, packet_id))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handshake {
    pub protocol_version: u8,
}
impl Handshake {
    pub fn new(protocol_version: u8) -> Handshake {
        Handshake { protocol_version }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Handshake> {
        let (_len, _id) = read_packet_header(t)?;
        let protocol_version = read_u8(t)?;
        Ok(Handshake::new(protocol_version))
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 2); // 2: 1 for ID, 1 for protocol_number
        write_vec_u8(&mut bytes, 0x00); // 0x00 Handshake
        write_vec_u8(&mut bytes, self.protocol_version);
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HandshakeResponse {}
impl HandshakeResponse {
    pub fn new() -> HandshakeResponse {
        HandshakeResponse {}
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<HandshakeResponse> {
        let (_len, _id) = read_packet_header(t)?;
        Ok(HandshakeResponse::new())
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 1); // 1: 1 for ID
        write_vec_u8(&mut bytes, 0x08); // 0x08 Handshake Response
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ping {}
impl Ping {
    pub fn new() -> Ping {
        Ping {}
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Ping> {
        let (_len, _id) = read_packet_header(t)?;
        Ok(Ping::new())
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 1); // 1: 1 for ID
        write_vec_u8(&mut bytes, 0x09); // 0x09 Ping
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {}
impl Pong {
    pub fn new() -> Pong {
        Pong {}
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Pong> {
        let (_len, _id) = read_packet_header(t)?;
        Ok(Pong::new())
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 1); // 1: 1 for ID
        write_vec_u8(&mut bytes, 0x09); // 0x10 Pong
        bytes
    }
}
