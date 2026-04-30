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
    let mut tsi = Tsi::new(args.tty, args.baud);

    match args.command {
        Command::Read { addr } => {
            println!("Reading from {addr:#X}...");
<<<<<<< HEAD
            println!("Read 0x{:X}", len);
            read_req(&mut port, tsi::Command::Read, addr, len as u64);
            let mut serial_buf: Vec<u8> = vec![0; len];
            port.read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            println!("Read 0x{}", hex::encode(&serial_buf));
        }
        Command::Write { addr, data, len } => {
            println!("Writing {data} to {addr:#X}...");
            //check if the first character is 0x
            let mut parsed_data: Vec<u8> = Vec::new();
            if data.starts_with("0x") {
                //parse it as hex
                parsed_data = hex::decode(data).expect("could not parse data");
            } else {
                //parse it as decimal
                let mut decimal_data = data.parse::<u64>().expect("could not parse data");
                //convert decimal to hex
                parsed_data = decimal_data.to_le_bytes().to_vec();
            }
            
            if let Some(len) = len {
                let extra_bytes = len as usize - parsed_data.len();
                parsed_data.extend(vec![0; extra_bytes]);
            }
            write_req(&mut port, tsi::Command::Write, addr, &parsed_data);
=======
            println!(
                "Read {:#010x}",
                tsi.read_word(addr).expect("failed to read")
            );
        }
        Command::Write { addr, data, len } => {
            println!("Writing {data} to {addr:#X}...");
            let mut data = hex::decode(data).expect("could not parse data");
            if let Some(len) = len {
                data.resize(len, 0);
            }
            tsi.write(addr, &data).expect("failed to write");
            println!("Write complete");
>>>>>>> origin/main
        }
    }
}
