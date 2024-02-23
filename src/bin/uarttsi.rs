use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::time::Duration;
use std::{io::Read, path::PathBuf};

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;
use termios::ffi::{cfsetspeed, tcgetattr};
use termios::*;

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

    let mut file = File::options()
        .read(true)
        .write(true)
        .open(args.tty)
        .expect("failed to open TTY");
    let mut tty = Termios::from_fd(file.as_raw_fd()).expect("failed to open TTY");

    tty.c_cflag &= !PARENB; // Clear parity bit, disabling parity (most common)
    tty.c_cflag &= !CSTOPB; // Clear stop field, only one stop bit used in communication (most common)
    tty.c_cflag &= !CSIZE; // Clear all the size bits
    tty.c_cflag |= CS8; // 8 bits per byte (most common)
    tty.c_cflag &= !termios::os::linux::CRTSCTS; // Disable RTS/CTS hardware flow control (most common)
    tty.c_cflag |= CREAD | CLOCAL; // Turn on READ & ignore ctrl lines (CLOCAL = 1)

    tty.c_lflag &= !ICANON;
    tty.c_lflag &= !ECHO; // Disable echo
    tty.c_lflag &= !ECHOE; // Disable erasure
    tty.c_lflag &= !ECHONL; // Disable new-line echo
    tty.c_lflag &= !ISIG; // Disable interpretation of INTR, QUIT and SUSP

    tty.c_iflag &= !(IXON | IXOFF | IXANY); // Turn off s/w flow ctrl
    tty.c_iflag &= !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL); // Disable any special handling of received bytes

    tty.c_oflag &= !OPOST; // Prevent special interpretation of output bytes (e.g. newline chars)
    tty.c_oflag &= !ONLCR; // Prevent conversion of newline to carriage return/line feed

    tty.c_cc[VTIME] = 0;
    tty.c_cc[VMIN] = 0;

    cfsetispeed(&mut tty, termios::os::linux::B115200).expect("could not set TTY baud rate");
    cfsetospeed(&mut tty, termios::os::linux::B115200).expect("could not set TTY baud rate");

    tcsetattr(file.as_raw_fd(), TCSANOW, &tty).unwrap();

    match args.command {
        Command::Read { addr, len } => {
            // TODO: Send length with address.
            println!("Reading from {addr:#X}...");
            write_req(&mut file, tsi::Command::Read, addr, &[]).expect("failed to write request");
            file.flush();
            let mut serial_buf: Vec<u8> = vec![0; 1];
            file.read_exact(serial_buf.as_mut_slice())
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
            write_req(&mut file, tsi::Command::Write, addr, &data)
                .expect("failed to write request");
        }
    }
}
