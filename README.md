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
