
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    //open the ttys in read and write mode
    let mut fp_out = File::options().read(true).write(true).open("/dev/ttys018")?;
    //initialize a hashmap to store the data
    let mut dict = std::collections::HashMap::new();
    let mut data = vec![0; 4];

    loop {
        // 48 byte array
        let mut byte = [0; 48];
        fp_out.read_exact(&mut byte)?;
        // get the address
        let addr = u64::from_le_bytes(byte[4..12].try_into().unwrap());
        // if the first byte is 1, perform a write
        if byte[0] == 1 {
            // get the data
            data = byte[12..].to_vec();
            // write the data to the address
            dict.insert(addr, data.clone());
            std::io::stdout().flush()?;
        } else {
            // get the data
            data = dict.get(&addr).unwrap().clone();
            // write the data back to the tty
            fp_out.write(&data)?;
            fp_out.flush()?;   
        } 
    }


    // loop {
    //     // 12 byte array
    //     let mut byte = [0; 48];
    //     fp_out.read_exact(&mut byte)?;
    //     // get the address
    //     let addr = u64::from_le_bytes(byte[4..12].try_into().unwrap());
    //     // if the first byte is 1, perform a write
    //     if byte[0] == 1 {
    //         // get the data
    //         data = &byte[12..];
    //         // write the data to the address
    //         dict.insert(addr, data);
    //     } else {
    //         // get the data
    //         data = dict.get(&addr).unwrap();
    //         // write the data back to the tty
    //         fp_out.write(data)?;
    //         fp_out.flush()?;   
    //     } 
        
    //     // fp_out.write(&byte[4..12])?;
    //     // fp_out.flush()?;
    //     // let byte = &byte[4..8];
    //     // // print the bytes in hex
    //     // for b in byte.iter() {
    //     //     print!("{:02x} ", b);
    //     // }
    //     // println!();
    //     // std::io::stdout().flush()?;
    // }

    Ok(())
}