use crate::helper::*;
use std::net::TcpStream;

pub fn read_packet_header(t: &mut TcpStream) -> std::io::Result<(u32, u8)> {
    let packet_length = read_u32(t)?;
    let packet_id = read_u8(t)?;
    Ok((packet_length, packet_id))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handshake {
    pub protocol_version: u8,
    pub next_state: u8,
}
impl Handshake {
    pub fn new(protocol_version: u8, next_state: u8) -> Handshake {
        Handshake {
            protocol_version,
            next_state,
        }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Handshake> {
        let protocol_version = read_u8(t)?;
        let next_state = read_u8(t)?;
        Ok(Handshake::new(protocol_version, next_state))
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 3); // 3: 1 for ID, 1 for protocol_number, 1 for next_state
        write_vec_u8(&mut bytes, 0x00); // 0x00 Handshake
        write_vec_u8(&mut bytes, self.protocol_version);
        write_vec_u8(&mut bytes, self.next_state);
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
        Ok(Pong::new())
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 1); // 1: 1 for ID
        write_vec_u8(&mut bytes, 0x10); // 0x10 Pong
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Disconnect {}
impl Disconnect {
    pub fn new() -> Disconnect {
        Disconnect {}
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<Disconnect> {
        Ok(Disconnect::new())
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 1); // 1: 1 for ID
        write_vec_u8(&mut bytes, 0x07); // 0x07 Disconnect
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactorRequest {
    pub target: u64,
    pub range_start: u64,
    pub range_end: u64,
}
impl FactorRequest {
    pub fn new(target: u64, range_start: u64, range_end: u64) -> FactorRequest {
        FactorRequest {
            target,
            range_start,
            range_end,
        }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<FactorRequest> {
        let target = read_u64(t)?;
        let range_start = read_u64(t)?;
        let range_end = read_u64(t)?;
        Ok(FactorRequest::new(target, range_start, range_end))
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 25); // 25: 1 for ID, 8 for target, 8 for range_start, 8 for range_end
        write_vec_u8(&mut bytes, 0x05); // 0x05 Factor Request
        write_vec_u64(&mut bytes, self.target);
        write_vec_u64(&mut bytes, self.range_start);
        write_vec_u64(&mut bytes, self.range_end);
        bytes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactorResponse {
    pub found_factor: bool,
    pub factor_value: u64,
}
impl FactorResponse {
    pub fn new(found_factor: bool, factor_value: u64) -> FactorResponse {
        FactorResponse {
            found_factor,
            factor_value,
        }
    }
    pub fn read(t: &mut TcpStream) -> std::io::Result<FactorResponse> {
        let found_factor = read_u8(t)? == 0x01;
        let factor_value = read_u64(t)?;
        Ok(FactorResponse::new(found_factor, factor_value))
    }
    pub fn write(&self, t: &mut TcpStream) -> std::io::Result<()> {
        write_bytes(t, &self.to_bytes())?;
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_u32(&mut bytes, 9); // 9: 1 for found_factor 8 for factor_value
        write_vec_u8(&mut bytes, 0x06); // 0x06 FactorResponse
        write_vec_u8(
            &mut bytes,
            match self.found_factor {
                true => 0x01,
                false => 0x00,
            },
        );
        write_vec_u64(&mut bytes, self.factor_value);
        bytes
    }
}
