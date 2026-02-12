use crate::{fifo::read_fifo, rumble::{create_effect, play_effect}};

mod rumble;
mod fifo;
mod utils;
use std::env;

use std::sync::mpsc;
use std::thread;


fn main() -> std::io::Result<()> {
    // Alter this or don't idk, that env is a monster :)
    let path: String = (env::home_dir().unwrap().to_str().unwrap().to_owned() + "/Documents/fifo/myfifo").to_string();
    
    let mut device = utils::get_device();
    let mut effect;

    let (tx, rx) = mpsc::channel();
    
    // Thread to read the fifo
    // The goal is to always have an up to date fifo even if effect lags as if we miss a read rumble gets stuck till we get a new read
    // Whereas this would mean that we always get the latest read even if we miss a read
    thread::spawn(move || {
        loop {
            let val: u16 = read_fifo(&path);
            tx.send(val).unwrap();
        }
    });

    loop {
        match rx.recv() {
            Ok(value) => {
                effect = create_effect(&mut device, value)?;
                play_effect(&mut effect)?;
            },
            Err(err) => {
                println!("{}", err)
            },
        }
        
    }
}