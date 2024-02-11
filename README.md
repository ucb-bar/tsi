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
