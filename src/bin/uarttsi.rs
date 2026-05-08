use std::{
    thread::{self, sleep},
    time::Duration,
};

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

use tsi::Tsi;

#[derive(Debug, Parser)]
#[clap(name = "uarttsi", version)]
pub struct Args {
    /// The TTY device.
    #[clap(short = 't', long)]
    tty: String,
    /// The TTY device.
    #[clap(short = 'b', long)]
    baud_rate: u32,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Help message for read.
    Read {
        #[clap(value_parser=maybe_hex::<u64>)]
        addr: u64,
        /// The desired read length in bytes.
        #[clap(short='l', long, value_parser=maybe_hex::<usize>, default_value_t=8)]
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
        /// If provided, zero-pads/truncates the write data to the given length. If length is not a multiple
        /// of 4, data will be additionally zero-padded to a multiple of 4 bytes.
        #[clap(short='l', long, value_parser=maybe_hex::<usize>)]
        len: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();

    println!("{} {}", args.tty, args.baud_rate);
    let mut tsi = Tsi::new(
        serialport::new(&args.tty, args.baud_rate)
            .timeout(Duration::from_millis(3000))
            .open()
            .expect("failed to open TTY"),
    );
    sleep(Duration::from_millis(500));

    match args.command {
        Command::Read { addr, len } => {
            println!("Reading from {addr:#X}...");
            println!(
                "Read {}",
                tsi.read(addr, len)
                    .expect("failed to read")
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        Command::Write { addr, data, len } => {
            println!("Writing {data} to {addr:#X}...");
            let mut data = hex::decode(data).expect("could not parse data");
            if let Some(len) = len {
                data.resize(len, 0);
            }
            tsi.write(addr, &data).expect("failed to write");
            thread::sleep(Duration::from_millis(10));
            println!("Write complete");
        }
    }
}
