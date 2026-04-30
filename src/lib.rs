use std::{io::Write, time::Duration};

use serialport::SerialPort;

const READ_OPCODE: u32 = 0;
const WRITE_OPCODE: u32 = 1;

pub fn write_req<W: Write>(w: &mut W, addr: u64, data: &[u8]) -> std::io::Result<()> {
    w.write_all(&WRITE_OPCODE.to_le_bytes())?;
    w.write_all(&addr.to_le_bytes())?;

    let num_words = std::cmp::max(data.len() / 4, 1);
    w.write_all(&(num_words - 1).to_le_bytes())?;

    let mut padded_data = Vec::with_capacity(4 * num_words);
    padded_data.extend_from_slice(data);
    padded_data.resize(4 * num_words, 0);

    write_chunks(w, padded_data)?;

    Ok(())
}

pub fn read_req<W: Write>(w: &mut W, command: Command, addr: u64, len: u64) -> std::io::Result<()> {
    w.write_all(&command.to_u32().to_le_bytes())?;
    w.write_all(&addr.to_le_bytes())?;
    w.write_all(&len.to_le_bytes())?;
    Ok(())
}

pub fn write_chunks<W: Write>(w: &mut W, data: &[u8]) -> std::io::Result<()> {
    let extra_bytes = data.len().div_ceil(4) * 4 - data.len();
    w.write_all(data)?;
    w.write_all(&vec![0; extra_bytes])?;
    Ok(())
}

#[derive(Debug)]
pub struct Tsi(Box<dyn SerialPort>);

impl Tsi {
    pub fn new(path: impl AsRef<str>, baud_rate: u32) -> Self {
        Self(
            serialport::new(path.as_ref(), baud_rate)
                .timeout(Duration::from_secs(3))
                .open()
                .expect("failed to open TTY"),
        )
    }

    pub fn read_word(&mut self, addr: u64) -> std::io::Result<u32> {
        write_req(&mut self.0, Command::Read, addr, &[])?;
        let mut serial_buf = [0; 4];
        self.0.read_exact(serial_buf.as_mut_slice())?;
        Ok(u32::from_le_bytes(serial_buf))
    }

    pub fn write(&mut self, addr: u64, data: &[u8]) -> std::io::Result<()> {
        write_req(&mut self.0, Command::Write, addr, data)
    }

    pub fn write_word(&mut self, addr: u64, data: u64) -> std::io::Result<()> {
        self.write(addr, &data.to_le_bytes())
    }
}
