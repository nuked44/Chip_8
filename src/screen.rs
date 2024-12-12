use crate::config::*;

pub mod tui;

pub trait Interface {
    fn new() -> Self;
    fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool;
    fn update_screen(&mut self);
    fn clear_screen(&mut self);
    fn get_key(&self, key: u8) -> bool;
    fn get_keys_pressed(&self) -> Vec<u8>;
    fn get_close_window(&self) -> bool;
    fn init(&self);
    fn stop(&self);
}
