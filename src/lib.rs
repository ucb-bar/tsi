use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Read,
    Write,
}

impl Command {
    fn to_u32(&self) -> u32 {
        match self {
            Command::Read => 0,
            Command::Write => 1,
        }
    }
}

pub fn write_req<W: Write>(
    w: &mut W,
    command: Command,
    addr: u64,
    data: &[u8],
) -> std::io::Result<()> {
    w.write_all(&command.to_u32().to_le_bytes())?;
    w.write_all(&addr.to_le_bytes())?;

    let num_words = std::cmp::max(data.len() / 4, 1);
    w.write_all(&(num_words - 1).to_le_bytes())?;

    write_chunks(w, data)?;

    Ok(())
}

pub fn write_chunks<W: Write>(w: &mut W, data: &[u8]) -> std::io::Result<()> {
    let extra_bytes = (data.len() + 3) / 4 * 4 - data.len();
    w.write_all(data)?;
    w.write_all(&vec![0; extra_bytes])?;
    Ok(())
}
