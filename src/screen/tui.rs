use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

use super::{Interface, SCREEN_HEIGHT, SCREEN_WIDTH};

// Keys for querty keyboard
const KEY_1: KeyCode = KeyCode::Char(super::KEY_1);
const KEY_2: KeyCode = KeyCode::Char(super::KEY_2);
const KEY_3: KeyCode = KeyCode::Char(super::KEY_3);
const KEY_4: KeyCode = KeyCode::Char(super::KEY_4);
const KEY_5: KeyCode = KeyCode::Char(super::KEY_5);
const KEY_6: KeyCode = KeyCode::Char(super::KEY_6);
const KEY_7: KeyCode = KeyCode::Char(super::KEY_7);
const KEY_8: KeyCode = KeyCode::Char(super::KEY_8);
const KEY_9: KeyCode = KeyCode::Char(super::KEY_9);
const KEY_0: KeyCode = KeyCode::Char(super::KEY_0);
const KEY_A: KeyCode = KeyCode::Char(super::KEY_A);
const KEY_B: KeyCode = KeyCode::Char(super::KEY_B);
const KEY_C: KeyCode = KeyCode::Char(super::KEY_C);
const KEY_D: KeyCode = KeyCode::Char(super::KEY_D);
const KEY_E: KeyCode = KeyCode::Char(super::KEY_E);
const KEY_F: KeyCode = KeyCode::Char(super::KEY_F);

pub struct Tui {
    pixel_bitmap: [bool; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
    close_window: bool,
}

impl Tui {
    fn set_pixel(&mut self, x: u8, y: u8, val: bool) -> bool {
        // Clip out of bounds
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return false;
        }
        let pixel = &mut self.pixel_bitmap[x as usize + y as usize * SCREEN_WIDTH as usize];
        let before = *pixel;
        *pixel ^= val;
        if !*pixel && before {
            return true;
        }
        false
    }

    pub fn key_to_u8(&self, key: Option<KeyCode>) -> Option<u8> {
        match key {
            Some(val) => match val {
                KEY_0 => Some(0x0),
                KEY_1 => Some(0x1),
                KEY_2 => Some(0x2),
                KEY_3 => Some(0x3),
                KEY_4 => Some(0x4),
                KEY_5 => Some(0x5),
                KEY_6 => Some(0x6),
                KEY_7 => Some(0x7),
                KEY_8 => Some(0x8),
                KEY_9 => Some(0x9),
                KEY_A => Some(0xA),
                KEY_B => Some(0xB),
                KEY_C => Some(0xC),
                KEY_D => Some(0xD),
                KEY_E => Some(0xE),
                KEY_F => Some(0xF),
                _ => None,
            },
            None => None,
        }
    }

    fn is_key_pressed(target_key: KeyCode) -> bool {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) = event::read().unwrap()
            {
                return code == target_key;
            }
        }
        false
    }

    fn print_to_term(buffer: [char; 2 * (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)]) {
        let mut stdout = io::stdout().lock();

        // Reset cursor to the top-left corner
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        match super::TUI_OUTPUT_MODE {
            super::TuiOutputMode::VecU8 => {
                let mut output =
                    Vec::<u8>::with_capacity(buffer.len() + 2 * SCREEN_HEIGHT as usize);

                for y in 0..SCREEN_HEIGHT as usize {
                    for x in 0..(2 * SCREEN_WIDTH) as usize {
                        let index = y * (2 * SCREEN_WIDTH as usize) + x;
                        output.push(buffer[index] as u8)
                    }
                    if y < SCREEN_HEIGHT as usize - 1 {
                        output.push(b'\r');
                        output.push(b'\n');
                    }
                }
                stdout.write_all(&output).unwrap();
            }
            super::TuiOutputMode::String => {
                let mut output = String::with_capacity(buffer.len() + 2 * SCREEN_HEIGHT as usize);

                for y in 0..SCREEN_HEIGHT as usize {
                    for x in 0..(2 * SCREEN_WIDTH) as usize {
                        let index = y * (2 * SCREEN_WIDTH as usize) + x;
                        output.push(buffer[index])
                    }
                    if y < SCREEN_HEIGHT as usize - 1 {
                        output.push_str("\r\n");
                    }
                }
                stdout.write_all(output.as_bytes()).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}

impl Interface for Tui {
    fn new() -> Self {
        Tui {
            pixel_bitmap: [false; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize],
            close_window: false,
        }
    }

    fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut pixel_erased: bool = false;
        let x = x % SCREEN_WIDTH;
        let y = y % SCREEN_HEIGHT;
        for (iteration, line) in sprite.iter().enumerate() {
            for bit in 0..8u8 {
                if self.set_pixel(
                    x + bit,
                    y + iteration as u8,
                    (line & (0b10000000 >> bit)) != 0,
                ) {
                    pixel_erased = true;
                }
            }
        }
        pixel_erased
    }

    fn update_screen(&mut self) {
        if Tui::is_key_pressed(KeyCode::Esc) {
            self.close_window = true;
        }

        let mut output_buffer = [' '; 2 * (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)];

        for (index, &pixel) in self.pixel_bitmap.iter().enumerate() {
            let y = index / SCREEN_WIDTH as usize;
            let x = index % SCREEN_WIDTH as usize;

            let output_index = y * (2 * SCREEN_WIDTH as usize) + (2 * x);

            let char = if pixel {
                super::PIXEL_ON
            } else {
                super::PIXEL_OFF
            };
            output_buffer[output_index] = char;
            output_buffer[output_index + 1] = char;
        }

        Tui::print_to_term(output_buffer);
    }

    fn clear_screen(&mut self) {
        for i in self.pixel_bitmap.iter_mut() {
            *i = false;
        }
        self.update_screen();
    }

    fn get_key(&self, key: u8) -> bool {
        match key {
            0x0 => Tui::is_key_pressed(KEY_0),
            0x1 => Tui::is_key_pressed(KEY_1),
            0x2 => Tui::is_key_pressed(KEY_2),
            0x3 => Tui::is_key_pressed(KEY_3),
            0x4 => Tui::is_key_pressed(KEY_4),
            0x5 => Tui::is_key_pressed(KEY_5),
            0x6 => Tui::is_key_pressed(KEY_6),
            0x7 => Tui::is_key_pressed(KEY_7),
            0x8 => Tui::is_key_pressed(KEY_8),
            0x9 => Tui::is_key_pressed(KEY_9),
            0xA => Tui::is_key_pressed(KEY_A),
            0xB => Tui::is_key_pressed(KEY_B),
            0xC => Tui::is_key_pressed(KEY_C),
            0xD => Tui::is_key_pressed(KEY_D),
            0xE => Tui::is_key_pressed(KEY_E),
            0xF => Tui::is_key_pressed(KEY_F),
            _ => false,
        }
    }

    fn get_keys_pressed(&self) -> Vec<u8> {
        todo!()
    }

    fn get_close_window(&self) -> bool {
        self.close_window
    }

    fn init(&self) {
        io::stdout()
            .execute(EnterAlternateScreen)
            .expect("Could not Enter alternate Screen");
        io::stdout().execute(cursor::Hide).unwrap();
        terminal::enable_raw_mode().expect("Could not enable raw mode");
    }

    fn stop(&self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        io::stdout()
            .execute(LeaveAlternateScreen)
            .expect("Could not leave Alternate Screen");
    }
}
