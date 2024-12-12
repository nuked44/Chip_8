#![allow(unused_variables)]

mod chip;
mod config;
mod inst;
mod screen;

use std::{
    env,
    fs::{self, File},
    io::Read,
};

use chip::Chip;
use config::*;
use screen::{tui::Tui, Interface};

fn get_file_in_bytes(filename: &String) -> Vec<u8> {
    let mut file = File::open(&filename).expect("file not found: {filename}");
    let metadata = fs::metadata(&filename).expect("unable to read metadata: {filename}");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read(&mut buffer)
        .expect("buffer overflow in main::get_file_in_bytes");

    buffer
}

fn main() {
    //Debug
    //env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("missing filepath argument");
    };

    let interface = Tui::new();

    let mut chip: Chip<Tui> = Chip::new(PROG_POS_START, interface);

    chip.load_prog(get_file_in_bytes(&args[1]));

    chip.init_interface();
    chip.run();
    chip.stop_interface();
}
