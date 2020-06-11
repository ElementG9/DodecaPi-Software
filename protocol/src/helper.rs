use std::io::prelude::{Read, Write};
use std::net::TcpStream;

// For writing to a TcpStream.
pub fn write_u8(t: &mut TcpStream, data: u8) -> std::io::Result<()> {
    t.write(&data.to_le_bytes())?;
    Ok(())
}
pub fn write_u16(t: &mut TcpStream, data: u16) -> std::io::Result<()> {
    t.write(&data.to_le_bytes())?;
    Ok(())
}
pub fn write_u32(t: &mut TcpStream, data: u32) -> std::io::Result<()> {
    t.write(&data.to_le_bytes())?;
    Ok(())
}
pub fn write_u64(t: &mut TcpStream, data: u64) -> std::io::Result<()> {
    t.write(&data.to_le_bytes())?;
    Ok(())
}
pub fn write_bytes(t: &mut TcpStream, data: &Vec<u8>) -> std::io::Result<()> {
    for b in data {
        write_u8(t, *b)?;
    }
    Ok(())
}
// For reading from a TcpStream.
pub fn read_u8(t: &mut TcpStream) -> std::io::Result<u8> {
    let mut buf = [0u8; 1];
    t.read_exact(&mut buf)?;
    Ok(u8::from_le_bytes(buf))
}
pub fn read_u16(t: &mut TcpStream) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    t.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}
pub fn read_u32(t: &mut TcpStream) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    t.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}
pub fn read_u64(t: &mut TcpStream) -> std::io::Result<u64> {
    let mut buf = [0u8; 8];
    t.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}
// For writing to a Vec<u8>.
pub fn write_vec_u8(v: &mut Vec<u8>, data: u8) {
    for b in &data.to_le_bytes() {
        v.push(*b);
    }
}
pub fn write_vec_u16(v: &mut Vec<u8>, data: u16) {
    for b in &data.to_le_bytes() {
        v.push(*b);
    }
}
pub fn write_vec_u32(v: &mut Vec<u8>, data: u32) {
    for b in &data.to_le_bytes() {
        v.push(*b);
    }
}
pub fn write_vec_u64(v: &mut Vec<u8>, data: u64) {
    for b in &data.to_le_bytes() {
        v.push(*b);
    }
}
pub fn write_vec_bytes(v: &mut Vec<u8>, data: &Vec<u8>) {
    for b in data {
        write_vec_u8(v, *b);
    }
}
// For reading from a Vec<u8>.
pub fn read_vec_u8(v: &mut Vec<u8>) -> Result<u8, ()> {
    if v.len() >= 1 {
        let data = [v[0]];
        *v = v[1..].into();
        Ok(u8::from_le_bytes(data))
    } else {
        Err(())
    }
}
pub fn read_vec_u16(v: &mut Vec<u8>) -> Result<u16, ()> {
    if v.len() >= 2 {
        let data = [v[0], v[1]];
        *v = v[2..].into();
        Ok(u16::from_le_bytes(data))
    } else {
        Err(())
    }
}
pub fn read_vec_u32(v: &mut Vec<u8>) -> Result<u32, ()> {
    if v.len() >= 4 {
        let data = [v[0], v[1], v[2], v[3]];
        *v = v[4..].into();
        Ok(u32::from_le_bytes(data))
    } else {
        Err(())
    }
}
pub fn read_vec_u64(v: &mut Vec<u8>) -> Result<u64, ()> {
    if v.len() >= 8 {
        let data = [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]];
        *v = v[8..].into();
        Ok(u64::from_le_bytes(data))
    } else {
        Err(())
    }
}

pub fn disconnect(t: &mut TcpStream) -> std::io::Result<()> {
    use std::net::Shutdown;
    crate::packet::Disconnect::new().write(t)?;
    t.shutdown(Shutdown::Both)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_vec_u8_works() {
        let mut v = vec![0x01, 0x03];
        write_vec_u8(&mut v, 0x05);
        assert_eq!(v, vec![0x01, 0x03, 0x05]);
    }
    #[test]
    fn write_vec_u16_works() {
        let mut v = vec![0x01, 0x03];
        write_vec_u16(&mut v, 0x0705);
        assert_eq!(v, vec![0x01, 0x03, 0x05, 0x07]);
    }
    #[test]
    fn write_vec_u32_works() {
        let mut v = vec![0x01, 0x03];
        write_vec_u32(&mut v, 0x07050000);
        assert_eq!(v, vec![0x01, 0x03, 0x00, 0x00, 0x05, 0x07]);
    }
    #[test]
    fn write_vec_u64_works() {
        let mut v = vec![0x01, 0x03];
        write_vec_u64(&mut v, 0x0705000000000000);
        assert_eq!(
            v,
            vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07]
        );
    }
    #[test]
    fn write_vec_bytes_works() {
        let mut v = vec![0x01, 0x03];
        write_vec_bytes(&mut v, &vec![0x05, 0x07]);
        assert_eq!(v, vec![0x01, 0x03, 0x05, 0x07]);
    }
    #[test]
    fn read_vec_u8_works() {
        let mut v = vec![0x01];
        let num1 = read_vec_u8(&mut v);
        let num2 = read_vec_u8(&mut v);
        assert_eq!(num1, Ok(0x01));
        assert_eq!(num2, Err(()));
    }
    #[test]
    fn read_vec_u16_works() {
        let mut v = vec![0x01, 0x03, 0x05];
        let num1 = read_vec_u16(&mut v);
        let num2 = read_vec_u16(&mut v);
        assert_eq!(num1, Ok(0x0301));
        assert_eq!(num2, Err(()));
    }
    #[test]
    fn read_vec_u32_works() {
        let mut v = vec![0x01, 0x03, 0x05, 0x07, 0x09];
        let num1 = read_vec_u32(&mut v);
        let num2 = read_vec_u32(&mut v);
        assert_eq!(num1, Ok(0x07050301));
        assert_eq!(num2, Err(()));
    }
    #[test]
    fn read_vec_u64_works() {
        let mut v = vec![0x01, 0x03, 0x05, 0x07, 0x09, 0x11, 0x13, 0x15];
        let num1 = read_vec_u64(&mut v);
        let num2 = read_vec_u64(&mut v);
        assert_eq!(num1, Ok(0x1513110907050301));
        assert_eq!(num2, Err(()));
    }
}
