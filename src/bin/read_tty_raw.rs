use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    //open the ttys in read and write mode
    let mut fp_out = File::options()
        .read(true)
        .write(true)
        .open("/dev/ttys027")?;
    //initialize a hashmap to store the data
    // let mut dict = std::collections::HashMap::new();
    // let mut data;

    loop {
        // 48 byte array
        let mut chunk = [0; 4];
        fp_out.read_exact(&mut chunk)?;
        println!("{:?}", chunk);
        // // get the address
        // let addr = u64::from_le_bytes(header[4..12].try_into().unwrap());
        // // if the first byte is 1, perform a write
        // if header[0] == 1 {
        //     // get the data
        //     data = byte[12..].to_vec();
        //     // write the data to the address
        //     dict.insert(addr, data.clone());
        //     std::io::stdout().flush()?;
        // } else {
        //     // get the data
        //     data = dict.get(&addr).unwrap().clone();
        //     // write the data back to the tty
        //     fp_out.write_all(&data)?;
        //     fp_out.flush()?;
        // }
    }
}

