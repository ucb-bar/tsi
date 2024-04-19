import sys

fp_out = open("/dev/ttys018", "r+b")

mem = {}

while True:
    bits = ''.join('{:02x}'.format(x) for x in fp_out.read(24))
    write = bits[1]
    addr = bits[8:16]
    data = bits[40:48]
    if write == "1":
        mem[addr] = data
    else:
        #write mem[addr] to tty. else write 00000000
        if addr in mem:
            print(int(mem[addr], 16).to_bytes(4, byteorder='big'))
            fp_out.write(int(mem[addr], 16).to_bytes(4, byteorder='big'))
        else:
            fp_out.write(b'\x00\x00\x00\x00')

    print(mem)

    sys.stdout.flush()
