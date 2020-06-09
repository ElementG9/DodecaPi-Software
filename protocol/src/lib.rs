pub struct Packet {
    pub id: u8,
    pub data: Vec<u8>,
}
impl Packet {
    pub fn new(id: u8) -> Packet {
        Packet {
            id,
            data: Vec::new(),
        }
    }
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let id_bytes = self.id.to_le_bytes();
        for b in &((id_bytes.len() + self.data.len()) as u32).to_le_bytes() {
            out.push(*b);
        }
        for b in &id_bytes {
            out.push(*b);
        }
        for b in &self.data {
            out.push(*b);
        }
        out
    }

    pub fn write_bool(&mut self, data: bool) {
        self.data.push(match data {
            true => 0x01,
            false => 0x00,
        });
    }
    pub fn write_u8(&mut self, data: u8) {
        self.data.push(data);
    }
    pub fn write_u16(&mut self, data: u16) {
        for b in &data.to_le_bytes() {
            self.data.push(*b);
        }
    }
    pub fn write_u32(&mut self, data: u32) {
        for b in &data.to_le_bytes() {
            self.data.push(*b);
        }
    }
    pub fn write_u64(&mut self, data: u64) {
        for b in &data.to_le_bytes() {
            self.data.push(*b);
        }
    }
    pub fn write_string(&mut self, data: String) {
        self.write_u32(data.len() as u32);
        for b in data.as_bytes() {
            self.write_u8(*b);
        }
    }
    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        for b in bytes {
            self.data.push(b);
        }
    }

    pub fn read_bool(index: usize, bytes: &Vec<u8>) -> bool {
        match Packet::read_u8(index, bytes) {
            0x00 => false,
            _ => true,
        }
    }
    pub fn read_u8(index: usize, bytes: &Vec<u8>) -> u8 {
        bytes[index]
    }
    pub fn read_u16(index: usize, bytes: &Vec<u8>) -> u16 {
        let b = [bytes[index], bytes[index + 1]];
        u16::from_le_bytes(b)
    }
    pub fn read_u32(index: usize, bytes: &Vec<u8>) -> u32 {
        let b = [
            bytes[index],
            bytes[index + 1],
            bytes[index + 2],
            bytes[index + 3],
        ];
        u32::from_le_bytes(b)
    }
    pub fn read_u64(index: usize, bytes: &Vec<u8>) -> u64 {
        let b = [
            bytes[index],
            bytes[index + 1],
            bytes[index + 2],
            bytes[index + 3],
            bytes[index + 4],
            bytes[index + 5],
            bytes[index + 6],
            bytes[index + 7],
        ];
        u64::from_le_bytes(b)
    }
    pub fn read_string(index: usize, bytes: &Vec<u8>) -> String {
        let str_len = Packet::read_u32(index, bytes);
        let mut str_bytes = Vec::new();
        for i in (index + 4)..(index + 4 + str_len as usize) {
            str_bytes.push(bytes[i]);
        }
        String::from_utf8_lossy(&str_bytes).to_string()
    }
}
