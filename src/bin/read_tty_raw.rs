use std::io::Read;
use std::time::Duration;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "uarttsi", version)]
pub struct Args {
    /// The TTY device.
    #[clap(short = 't', long)]
    tty: String,
    /// The TTY device.
    #[clap(short = 'b', long)]
    baud_rate: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut port = serialport::new(args.tty, args.baud_rate)
        .timeout(Duration::from_secs(3))
        .open()
        .expect("failed to open TTY");

    let mut buf = [0u8; 1024];

    loop {
        match port.read(&mut buf) {
            Ok(n) if n > 0 => {
                for b in &buf[..n] {
                    print!("{:02x} ", b);
                }
                println!();
            }
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // no data yet
            }
            Err(e) => return Err(e.into()),
        }
    }
}
