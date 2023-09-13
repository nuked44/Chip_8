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
    pixel_drawbuffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

#[allow(dead_code)]
impl Screen {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
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
            pixel_drawbuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, val: bool) -> bool {
        let pixel = &mut self.pixel_bitmap[x as usize + y as usize * SCREEN_WIDTH];
        let before = *pixel;
        *pixel = *pixel != val;
        if !*pixel && before {
            return true;
        }
        false
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut pixel_erased: bool = false;
        for (i, line) in sprite.iter().enumerate() {
            for bit in 0..8u8 {
                if self.set_pixel(x + bit, y + i as u8, (line & (0b10000000 >> bit)) != 0) {
                    pixel_erased = true;
                }
            }
        }
        pixel_erased
    }

    pub fn update_pixel_drawbuffer(&mut self, fg: &Color, bg: &Color) {
        for pixel in self.pixel_bitmap.iter().enumerate() {
            let c_selec: &Color = if *pixel.1 { fg } else { bg };
            let draw_pixel: u32 =
                ((c_selec.r as u32) << 16) | ((c_selec.g as u32) << 8) | (c_selec.b as u32);
            self.pixel_drawbuffer[pixel.0] = draw_pixel;
        }
    }

    pub fn update_screen(&mut self) {
        self.window
            .update_with_buffer(&self.pixel_drawbuffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("coudn't update screen");
    }

    pub fn clear_screen(&mut self) {
        for i in self.pixel_bitmap.iter_mut() {
            *i = false;
        }
    }
}
