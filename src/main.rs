#![allow(unused_variables)]

mod chip;
mod config;
mod inst;

use std::{
    env,
    fs::{self, File},
    io::Read, time::{Duration, Instant},
};

use chip::Chip;
use config::*;
use macroquad::{color, window::{clear_background, next_frame, Conf}};

fn get_file_in_bytes(filename: &String) -> Vec<u8> {
    let mut file = File::open(filename).expect("file not found: {filename}");
    let metadata = fs::metadata(filename).expect("unable to read metadata: {filename}");
    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read_exact(&mut buffer)
        .expect("buffer overflow in main::get_file_in_bytes");

    buffer
}

fn conf() -> Conf {
    Conf {
        window_title: "Chip-8 emulator".to_owned(),
        window_resizable: false,
        window_width: (SCREEN_WIDTH as i32*WINDOW_SCALE as i32),
        window_height: (SCREEN_HEIGHT as i32*WINDOW_SCALE as i32),
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    //Debug
    //env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("missing filepath argument");
    };

    let mut chip: Chip = Chip::new(PROG_POS_START);

    chip.load_prog(get_file_in_bytes(&args[1]));

    let target_frame_time = Duration::from_secs_f64(1f64 / SCREEN_REFRESH_RATE as f64);
        let mut last_frame = Instant::now();

        loop {
            clear_background(color::Color::from_hex(PIXEL_OFF_COLOR));
            chip.handle_input();
            if Instant::now() - last_frame >= target_frame_time {
                last_frame = Instant::now();
                // Get pressed keys
                
                // Execute next instructions for the frame
                for _ in 0..(INSTRUCTION_FREQUENCY / SCREEN_REFRESH_RATE) {
                    chip.execute_inst();
                }
                
                // Update timers
                chip.decrement_delay_timer();
                chip.decrement_sound_timer();
                
            }
            // Update screen
            chip.render_display_buffer();
            next_frame().await;
        }
}
