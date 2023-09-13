use minifb::Key;

use crate::screen::Color;

pub const FONT_POS_START: usize = 0x50;
pub const PROG_POS_START: usize = 0x200;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

    
pub const FOREGROUND: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub const BACKGROUND: Color = Color { r: 0, g: 0, b: 0 };

// ---------- Keys ----------

// Keys for querty keyboard
pub const KEY_1: Key = Key::Key1;
pub const KEY_2: Key = Key::Key2;
pub const KEY_3: Key = Key::Key3;
pub const KEY_4: Key = Key::Q;
pub const KEY_5: Key = Key::W;
pub const KEY_6: Key = Key::E;
pub const KEY_7: Key = Key::A;
pub const KEY_8: Key = Key::S;
pub const KEY_9: Key = Key::D;
pub const KEY_0: Key = Key::X;
pub const KEY_A: Key = Key::Z;
pub const KEY_B: Key = Key::C;
pub const KEY_C: Key = Key::Key4;
pub const KEY_D: Key = Key::R;
pub const KEY_E: Key = Key::F;
pub const KEY_F: Key = Key::V;

/* Keys for hex keyboard
pub const KEY_1: Key = Key::Key1;
pub const KEY_2: Key = Key::Key2;
pub const KEY_3: Key = Key::Key3;
pub const KEY_4: Key = Key::Key4;
pub const KEY_5: Key = Key::Key5;
pub const KEY_6: Key = Key::Key6;
pub const KEY_7: Key = Key::Key7;
pub const KEY_8: Key = Key::Key8;
pub const KEY_9: Key = Key::Key9;
pub const KEY_0: Key = Key::Key0;
pub const KEY_A: Key = Key::A;
pub const KEY_B: Key = Key::B;
pub const KEY_C: Key = Key::C;
pub const KEY_D: Key = Key::D;
pub const KEY_E: Key = Key::E;
pub const KEY_F: Key = Key::F;
*/
