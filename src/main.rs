mod chip;
mod inst;
mod screen;
mod test;

use chip::Chip;
use screen::Color;

fn main() {
    let fg:Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let bg:Color = Color {
        r: 0,
        g: 0,
        b: 0,
    };
    let mut chip: Chip = Chip::new();
    chip.set_byte(0, 0x69);
    let x: u8 = chip.get_byte(0);
    chip.set_byte(0xFFF, x);
    chip.screen.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while chip.screen.window.is_open() {
        chip.screen.update_drawbuffer(&fg, &bg);
        chip.screen.update_screen();
    }
}
