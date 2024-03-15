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

```
socat -d -d pty,raw,echo=0 pty,raw,echo=0
```

Then, use `utils/read_tty_raw.py` to listen on one of the created pseudo-TTYs by modifying the `fp_out`
variable:

```
fp_out = open("/dev/ttys011", "rb")
```

Then, run the script using python:

```
python3 read_tty_raw.py
```

When you write to the other pseudo-TTY, the raw bytes should show up in the terminal where you ran the 
Python scripts.

On Mac OS, you will need to specify a baud rate of zero when reading/writing pseudo-TTYs:

```
./target/debug/uarttsi -t /dev/ttys011 -b 0 write 0x1000 deadbeef
```


## Design

Default baud rate is 115200.

### Read commands

Takes in an address as a mandatory position argument. Can output read data as raw bytes or hex display.
Length provided as optional flag, default is 4.

### Write commands

Takes in an address as a mandatory position argument. Can take data as hex or decimal using same strategy as address.
Length provided as mandatory flag.

### Write binary command

Takes in an address and file as a mandatory position argument. Option to reset core and start executing.

### Things to debug
- Why does FPGA get reset
- Why does FPGA not work on lab computer
