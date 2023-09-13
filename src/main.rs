mod chip;
mod config;
mod inst;
mod screen;
mod test;

use std::{
    env,
    fs::{self, File},
    io::Read,
    time::{Duration, Instant},
};

use chip::Chip;
use config::*;
use minifb::Key;

fn get_file_in_bytes(filename: &String) -> Vec<u8> {
    let mut file = File::open(&filename).expect("file not found: {filename}");
    let metadata = fs::metadata(&filename).expect("unable to read metadata: {filename}");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read(&mut buffer)
        .expect("buffer overflow in main::get_file_in_bytes");

    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String;
    if args.len() > 1 {
        file_path = args[1].clone();
    } else {
        panic!("missing filepath argument");
    };

    let mut chip: Chip = Chip::new();

    let prog = get_file_in_bytes(&file_path);
    chip.load_prog(prog);
    chip.pc = PROG_POS_START as u16;

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while chip.screen.window.is_open() && !chip.screen.window.is_key_down(Key::Escape) {
        let keys_pressed = chip.screen.window.get_keys_pressed(minifb::KeyRepeat::Yes);
        let key = if !keys_pressed.is_empty() {
            Some(keys_pressed[0])
        } else {
            None
        };

        let chip8_key = chip.key_to_u8(key);
        if chip8_key.is_some()
            || Instant::now() - last_key_update_time >= Duration::from_millis(200)
        {
            last_key_update_time = Instant::now();
            chip.set_keyboard(chip8_key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip.execute_inst();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            last_display_time = Instant::now();
            chip.screen
                .update_pixel_drawbuffer(&FOREGROUND, &BACKGROUND);
            chip.screen.update_screen();
        }
    }
}
