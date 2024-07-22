
pub trait Display {
    fn set_pixel(&mut self, x: u8, y: u8, val: bool) -> bool;
    fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool;
    fn update_screen(&mut self);
    fn clear_screen(&mut self);
}