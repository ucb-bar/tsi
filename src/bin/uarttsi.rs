use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

use tsi::write_req;

#[derive(Debug, Parser)]
#[clap(name = "uarttsi", version)]
pub struct Args {
    /// The TTY device.
    #[clap(short = 't', long)]
    tty: String,
    /// The TTY device.
    #[clap(short = 'b', long)]
    baud: u32,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Help message for read.
    Read {
        #[clap(value_parser=maybe_hex::<u64>)]
        addr: u64,
        /// The desired read length in bytes. Is rounded up to the nearest multiple of 4.
        #[clap(short='l', long, value_parser=maybe_hex::<usize>, default_value="4")]
        len: usize,
    },
    /// Help message for write.
    Write {
        #[clap(value_parser=maybe_hex::<u64>)]
        addr: u64,
        /// The data to write, as a byte array in hex.
        ///
        /// If no length provided, data is zero-padded to the nearest multiple of 4 bytes.
        data: String,
        /// The desired write length in bytes.
        ///
        /// If provided, zero-pads the write data to the given length. If length is not a multiple
        /// of 4, data will be additionally zero-padded to a multiple of 4 bytes.
        #[clap(short='l', long, value_parser=maybe_hex::<usize>)]
        len: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();

    println!("{} {}", args.tty, args.baud);
    let mut port = serialport::new(args.tty, args.baud)
        .timeout(Duration::from_secs(3))
        .open()
        .expect("failed to open TTY");

    match args.command {
        Command::Read { addr, len } => {
            println!("Reading from {addr:#X}...");
            write_req(&mut port, tsi::Command::Read, addr, &[]);
            let mut serial_buf: Vec<u8> = vec![0; len];
            port.read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            println!("Read 0x{}", hex::encode(&serial_buf));
        }
        Command::Write { addr, data, len } => {
            println!("Writing {data} to {addr:#X}...");
            let mut data = hex::decode(data).expect("could not parse data");
            if let Some(len) = len {
                let extra_bytes = len as usize - data.len();
                data.extend(vec![0; extra_bytes]);
            }
            write_req(&mut port, tsi::Command::Write, addr, &data);
        }
    }
}
