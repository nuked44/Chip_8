mod chip;
mod config;
mod inst;
mod screen;

use std::{
    env,
    fs::{self, File},
    io::Read,
    time::{Duration, Instant},
};

use chip::Chip;
use config::*;
use minifb::Key;
use screen::Screen;

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
    if args.len() <= 1 {
        panic!("missing filepath argument");
    };

    let screen = Screen::new(
        "Chip-8",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &FOREGROUND,
        &BACKGROUND,
    );
    let mut chip: Chip = Chip::new(PROG_POS_START, screen);

    chip.load_prog(get_file_in_bytes(&args[1]));

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();
    let mut last_timer_decrement = Instant::now();

    while chip.screen.window.is_open() && !chip.screen.window.is_key_down(Key::Escape) {
        let keys_pressed = chip.screen.window.get_keys_pressed(minifb::KeyRepeat::Yes);
        let key = if !keys_pressed.is_empty() {
            Some(keys_pressed[0])
        } else {
            None
        };

        // Update keyboard when key is pressed or after 200ms
        let chip8_key = chip.key_to_u8(key);
        if chip8_key.is_some()
            || Instant::now() - last_key_update_time >= Duration::from_millis(200)
        {
            last_key_update_time = Instant::now();
            chip.keyboard = chip8_key;
        }

        // Execute next instruction every 2ms
        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            last_instruction_run_time = Instant::now();
            chip.execute_inst();
        }

        // Update screen every 10ms
        if Instant::now() - last_display_time > Duration::from_millis(10) {
            last_display_time = Instant::now();
            chip.screen.update_screen();
        }

        // Update Sound and Delay timer
        if Instant::now() - last_timer_decrement > Duration::from_micros(16666) {
            if chip.delay_timer > 0 {
                chip.delay_timer -= 1;
            }
            if chip.sound_timer > 0 {
                chip.sound_timer -= 1;
            }
            last_timer_decrement = Instant::now();
        }
    }
}
