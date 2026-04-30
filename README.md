# TSI

A standalone implementation of the Tethered Serial Interface (TSI).

TSI involves sending chunks of 32 bits. An operation typically looks like this:

```
| CMD | ADDR % 32 | ADDR >> 32 | LEN % 32 | LEN >> 32 | DATA CHUNK 1 | DATA CHUNK 2 | ... |
```

## Usage

```
Usage: uarttsi --tty <TTY> --baud <BAUD> <COMMAND>

Commands:
  read   Help message for read
  write  Help message for write
  help   Print this message or the help of the given subcommand(s)

Options:
  -t, --tty <TTY>
  -b, --baud <BAUD>
  -h, --help         Print help
  -V, --version      Print version
```

## Testing

To build the debug binary, run `cargo build`. The binary will be located in `target/debug/uarttsi`.

To create a pseudo-TTY for testing purposes, run the following:

```bash
socat -d -d pty,raw,echo=0 pty,raw,echo=0
```

Then, use the `read_tty_raw` binary to listen on one of the created pseudo-TTYs:

```bash
./target/debug/read_tty_raw -t /dev/ttys012 -b 0
```

When you write to the other pseudo-TTY, the raw bytes should show up in the `read_tty_raw` terminal.

On Mac OS, you will need to specify a baud rate of zero when reading/writing pseudo-TTYs:

```
./target/debug/uarttsi -t /dev/ttys011 -b 0 write 0x1000 deadbeef
```

To test that reads are working, it may be useful to make another binary utility to simulate a TSI memory.
