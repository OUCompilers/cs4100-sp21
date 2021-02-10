use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use byteorder::{ByteOrder,BigEndian,LittleEndian};

//Create, write, then read a binary file.
//To display the resulting file test.o, do:
// od --endian big -A x -x test.o
fn main() -> std::io::Result<()> {
    let mut bytes: Vec<u8> = vec![104, 101, 108, 108, 111, 33];

    //Create file
    let mut file = OpenOptions::new()
        .read(true).write(true).open("test.o")?;

    //Write bytes 0x48, then 0x45 to the beginning of the vector
    BigEndian::write_u16(&mut bytes, 0x48_45);

    //Write bytes 0x45, then 0x48 to the beginning of the vector
    //LittleEndian::write_u16(&mut bytes, 0x48_45);
    
    //Write bytes
    file.write_all(&bytes[..])?;

    //Seek back to beginning of file
    file.seek(SeekFrom::Start(0))?;

    //Read file into buf
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    //Print out each byte
    for c in buf {
        println!("{:x}", c);
    };

    Ok(())
}
