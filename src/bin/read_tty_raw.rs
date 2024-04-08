
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    //open the ttys in read and write mode
    let mut fp_out = File::options().read(true).write(true).open("/dev/ttys005")?;

    loop {
        // 12 byte array
        let mut byte = [0; 12];
        fp_out.read_exact(&mut byte)?;
        // write sixth byte back to the tty
        fp_out.write(&byte[5..6])?;
        fp_out.flush()?;
        // print the bytes in hex
        for b in byte.iter() {
            print!("{:02x} ", b);
        }
        std::io::stdout().flush()?;
    }

    Ok(())
}