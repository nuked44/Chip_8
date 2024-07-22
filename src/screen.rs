use crate::config::*;
use minifb::{Window, WindowOptions};
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Screen {
    pub window: Window,
    pub pixel_bitmap: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    foreground: u32,
    background: u32,
}

#[allow(dead_code)]
impl Screen {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
        foreground: &Color,
        background: &Color,
    ) -> Self {
        Screen {
            window: Window::new(
                name,
                width,
                height,
                WindowOptions {
                    scale: minifb::Scale::X16,
                    scale_mode: minifb::ScaleMode::Stretch,
                    ..WindowOptions::default()
                },
            )
            .expect("unable to initialize window"),
            pixel_bitmap: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            foreground: ((foreground.r as u32) << 16)
                | ((foreground.g as u32) << 8)
                | (foreground.b as u32),
            background: ((background.r as u32) << 16)
                | ((background.g as u32) << 8)
                | (background.b as u32),
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, val: bool) -> bool {
        if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
            return false;
        }
        let pixel = &mut self.pixel_bitmap[x as usize + y as usize * SCREEN_WIDTH];
        let before = *pixel;
        *pixel = *pixel ^ val;
        if !*pixel && before {
            return true;
        }
        false
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut pixel_erased: bool = false;
        let x = x % SCREEN_WIDTH as u8;
        let y = y % SCREEN_HEIGHT as u8;
        for (i, line) in sprite.iter().enumerate() {
            for bit in 0..8u8 {
                if self.set_pixel(x + bit, y + i as u8, (line & (0b10000000 >> bit)) != 0) {
                    pixel_erased = true;
                }
            }
        }
        pixel_erased
    }

    pub fn update_screen(&mut self) {
        let mut draw_buffer = [0u32; SCREEN_WIDTH * SCREEN_HEIGHT];
        for pixel in self.pixel_bitmap.iter().enumerate() {
            let color = if *pixel.1 {
                self.foreground
            } else {
                self.background
            };
            draw_buffer[pixel.0] = color;
        }
        self.window
            .update_with_buffer(&draw_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("coudn't update screen");
    }

    pub fn clear_screen(&mut self) {
        for i in self.pixel_bitmap.iter_mut() {
            *i = false;
        }
    }
}
