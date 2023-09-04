use minifb::{Window, WindowOptions};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Screen {
    pub window: Window,
    pub pixel_bitmap: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pixel_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

#[allow(dead_code)]
impl Screen {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        return Screen {
            window: Window::new(name, width, height, WindowOptions{
                scale: minifb::Scale::X16,
                scale_mode: minifb::ScaleMode::Stretch,
                ..WindowOptions::default()
            }).expect("unable to initialize window"),
            pixel_bitmap: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pixel_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
    }

    pub fn update_drawbuffer(&mut self, fg: &Color, bg: &Color) {
        for pixel in self.pixel_bitmap.iter().enumerate() {
            let c_selec: &Color = if *pixel.1 { fg } else { bg };
            let draw_pixel: u32 =
                0 | ((c_selec.r as u32) << 16) | ((c_selec.g as u32) << 8) | (c_selec.b as u32);
            self.pixel_buffer[pixel.0] = draw_pixel;
        }
    }

    pub fn update_screen(&mut self) {
        self.window
            .update_with_buffer(&self.pixel_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("coudn't update screen");
    }

    pub fn clear_screen(&mut self) {
        for i in self.pixel_bitmap.iter_mut(){
            *i = false;
        }
    }
}
