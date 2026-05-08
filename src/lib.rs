use std::io::Write;

use serialport::SerialPort;

const READ_OPCODE: u32 = 0;
const WRITE_OPCODE: u32 = 1;

pub fn num_words(len: u64) -> u64 {
    std::cmp::max(len.div_ceil(4), 1)
}

pub fn read_req<W: Write>(w: &mut W, addr: u64, len: u64) -> std::io::Result<()> {
    let mut buf = [0u8; 20];
    buf[0..4].copy_from_slice(&READ_OPCODE.to_le_bytes());
    buf[4..12].copy_from_slice(&addr.to_le_bytes());
    buf[12..20].copy_from_slice(&(num_words(len) - 1).to_le_bytes());
    w.write_all(&buf)
}

pub fn write_req<W: Write>(w: &mut W, addr: u64, data: &[u8]) -> std::io::Result<()> {
    let num_words = num_words(data.len() as u64);
    let extra_bytes = num_words as usize * 4 - data.len();
    let mut buf = Vec::with_capacity(20 + data.len() + extra_bytes);
    buf.extend_from_slice(&WRITE_OPCODE.to_le_bytes());
    buf.extend_from_slice(&addr.to_le_bytes());
    buf.extend_from_slice(&(num_words - 1).to_le_bytes());
    buf.extend_from_slice(data);
    buf.extend_from_slice(&vec![0; extra_bytes]);
    w.write_all(&buf)
}

#[derive(Debug)]
pub struct Tsi(Box<dyn SerialPort>);

impl Tsi {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self(port)
    }

    pub fn read(&mut self, addr: u64, len: usize) -> std::io::Result<Vec<u8>> {
        read_req(&mut self.0, addr, len as u64)?;
        let mut serial_buf = vec![0u8; len];
        self.0.read_exact(serial_buf.as_mut_slice())?;
        Ok(serial_buf)
    }

    pub fn read_word(&mut self, addr: u64) -> std::io::Result<u64> {
        Ok(u64::from_le_bytes(self.read(addr, 8)?.try_into().unwrap()))
    }

    pub fn write(&mut self, addr: u64, data: &[u8]) -> std::io::Result<()> {
        write_req(&mut self.0, addr, data)
    }

    pub fn write_word(&mut self, addr: u64, data: u64) -> std::io::Result<()> {
        self.write(addr, &data.to_le_bytes())
    }
}
