use std::fs::{File, OpenOptions};
use std::io::Read;

pub fn read_fifo_twelve_bytes(path: &String) -> std::io::Result<u16> {
    let mut fifo_file: File = OpenOptions::new().read(true).open(path)?;

    // The buffer is size 6 because we mimic the standard
    // If you change the standard change this
    let mut buffer = [0u8; 6];
    fifo_file.read_exact(&mut buffer)?;
    //println!("{:?}", buffer); Better than just printing the buffer, as these are raw bytes
    return Ok(buffer[5] as u16);
}

pub fn read_fifo(path:&String) -> u16 {
    read_fifo_twelve_bytes(path).unwrap() // You might think this is dumb but this makes the main file cleaner
}