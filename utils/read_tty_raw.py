import sys

fp_out = open("/dev/ttys018", "r+b")

# stores data in word-addressable memory
mem = {}

while True:
    #TODO: need to change the way we read so we can read variable length commands from tty so we can write variable length data
    bits = ''.join('{:02x}'.format(x) for x in fp_out.read(24))
    print(bits)
    write = bits[1]
    addr = bits[8:16]
    len = bits[16:24]
    data = bits[40:48]
    #TODO: implement len in backend so we can store bytes in multiple of 4 (just some dictionary arithmetic)
    if write == "1":
        mem[addr] = data
    else:
        #write mem[addr] to tty. else write 00000000
        if addr in mem:
            print(int(mem[addr], 16).to_bytes(4, byteorder='big'))
            fp_out.write(int(mem[addr], 16).to_bytes(4, byteorder='big'))
        else:
            fp_out.write(b'\x00\x00\x00\x00')

    # print(mem)

    sys.stdout.flush()
