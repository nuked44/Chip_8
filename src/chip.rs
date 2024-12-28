use macroquad::{color, input, shapes::draw_rectangle};
use rand::Rng;

use crate::{
    config::*,
    inst::{hex_to_inst, Inst},
};

pub struct Chip {
    memory: [u8; MEMSIZE],
    pc: u16,
    registers: [u8; 16],
    i: u16,
    stack: [u16; 16],
    stackpointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    keymap: [bool; 16],
    display_buffer: [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
}

#[allow(dead_code)]
impl Chip {
    pub fn new(prog_counter: u16) -> Self {
        Self {
            memory: [0; MEMSIZE],
            pc: prog_counter,
            registers: [0; 16],
            i: 0,
            stack: [0; 16],
            stackpointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keymap: [false; 16],
            display_buffer: [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
        }
    }

    pub fn load_prog(&mut self, prog: Vec<u8>) {
        for (i, inst_part) in prog.iter().enumerate() {
            self.memory[PROG_POS_START as usize + i] = *inst_part;
        }
    }

    pub fn execute_inst(&mut self) {
        let val: u16 = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[(self.pc + 1) as usize] as u16;
        let inst: Inst = hex_to_inst(val);
        match inst {
            Inst::Empty => self.pc += 2,
            Inst::Cls => {
                self.clear_screen();
                self.pc += 2;
            }
            Inst::Ret => {
                self.stackpointer -= 1;
                self.pc = self.stack[self.stackpointer as usize];
                self.pc += 2;
            }
            Inst::Jmp { addr } => {
                self.pc = addr;
            }
            Inst::Call { addr } => {
                self.stack[self.stackpointer as usize] = self.pc;
                self.stackpointer += 1;
                self.pc = addr;
            }
            Inst::SV { vx, byte } => {
                if self.registers[vx as usize] == byte {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SnV { vx, byte } => {
                if self.registers[vx as usize] != byte {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SR { vx, vy } => {
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::LdV { vx, byte } => {
                self.registers[vx as usize] = byte;
                self.pc += 2;
            }
            Inst::AddV { vx, byte } => {
                let (res, _) = self.registers[vx as usize].overflowing_add(byte);
                self.registers[vx as usize] = res;
                self.pc += 2;
            }
            Inst::LdR { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize];
                self.pc += 2;
            }
            Inst::OrR { vx, vy } => {
                let val = self.registers[vx as usize] | self.registers[vy as usize];
                self.registers[vx as usize] = val;
                self.registers[0xF] = 0;
                self.pc += 2;
            }
            Inst::AndR { vx, vy } => {
                let val = self.registers[vx as usize] & self.registers[vy as usize];
                self.registers[vx as usize] = val;
                self.registers[0xF] = 0;
                self.pc += 2;
            }
            Inst::XorR { vx, vy } => {
                let val = self.registers[vx as usize] ^ self.registers[vy as usize];
                self.registers[vx as usize] = val;
                self.registers[0xF] = 0;
                self.pc += 2;
            }
            Inst::AddR { vx, vy } => {
                let (res, vf) =
                    self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                self.registers[vx as usize] = res;
                self.registers[0xF] = vf as u8;
                self.pc += 2;
            }
            Inst::SubR { vx, vy } => {
                let (res, vf) =
                    self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);
                self.registers[vx as usize] = res;
                self.registers[0xF] = !vf as u8;
                self.pc += 2;
            }
            Inst::Shr { vx, vy } => {
                let val = self.registers[vy as usize];
                self.registers[vx as usize] = val >> 1;
                self.registers[0xF] = val & 0x1;
                self.pc += 2;
            }
            Inst::SubnR { vx, vy } => {
                let (res, vf) =
                    self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);
                self.registers[vx as usize] = res;
                self.registers[0xF] = !vf as u8;
                self.pc += 2;
            }
            Inst::Shl { vx, vy } => {
                let val = self.registers[vy as usize];
                self.registers[vx as usize] = val << 1;
                self.registers[0xF] = (val >> 7) & 0x1;
                self.pc += 2;
            }
            Inst::SnR { vx, vy } => {
                self.pc += 2;
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.pc += 2;
                }
            }
            Inst::LdI { addr } => {
                self.i = addr;
                self.pc += 2;
            }
            Inst::JpV0 { addr } => {
                self.pc = addr + self.registers[0x0] as u16;
            }
            Inst::Rnd { vx, byte } => {
                let val = (rand::thread_rng().gen_range(0..=255) as u8) & byte;
                self.registers[vx as usize] = val;
                self.pc += 2;
            }
            Inst::Disp { vx, vy, n } => {
                let mut sprite_buffer: Vec<u8> = Vec::new();
                for i in 0..n {
                    sprite_buffer.push(self.memory[(self.i + i as u16) as usize]);
                }
                if self.draw_sprite_to_display_buffer(
                    self.registers[vx as usize],
                    self.registers[vy as usize],
                    sprite_buffer,
                ) {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
                self.pc += 2;
            }
            Inst::SKp { vx } => {
                let target = self.registers[vx as usize];
                if self.is_key_pressed(target) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SKnp { vx } => {
                let target = self.registers[vx as usize];
                if !self.is_key_pressed(target) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::LdRDt { vx } => {
                self.registers[vx as usize] = self.delay_timer;
                self.pc += 2;
            }
            Inst::LdRKp { vx } => {
                for i in 0..16 {
                    if self.is_key_pressed(i) {
                        self.registers[vx as usize] = i;
                        self.pc += 2;
                        break;
                    }
                }
            }
            Inst::LdDtR { vx } => {
                self.delay_timer = self.registers[vx as usize];
                self.pc += 2;
            }
            Inst::LdStR { vx } => {
                self.sound_timer = self.registers[vx as usize];
                self.pc += 2;
            }
            Inst::AddRI { vx } => {
                let val = self.i + self.registers[vx as usize] as u16;
                self.i = val;
                self.pc += 2;
            }
            Inst::LdIF { vx } => {
                let x = self.registers[vx as usize];
                self.i = (FONT_POS_START + 5 * x as usize) as u16;
                self.pc += 2;
            }
            Inst::LdBCDR { vx } => {
                let val = self.registers[vx as usize];
                self.memory[self.i as usize] = val / 100;
                self.memory[(self.i + 1) as usize] = (val % 100) / 10;
                self.memory[(self.i + 2) as usize] = (val % 100) % 10;
                self.pc += 2;
            }
            Inst::LdIR { vx } => {
                let i = self.i;
                for x in 0..=vx {
                    self.memory[(i + x as u16) as usize] = self.registers[x as usize];
                }
                self.i = i + vx as u16 + 1;
                self.pc += 2;
            }
            Inst::LdRI { vx } => {
                let i = self.i;
                for x in 0..=vx {
                    self.registers[x as usize] = self.memory[(i + x as u16) as usize];
                }
                self.i = i + vx as u16 + 1;
                self.pc += 2;
            }
        }
    }

    pub fn decrement_delay_timer(&mut self) {
        let decrement = |timer: &mut u8| *timer = (*timer).saturating_sub(1);
        decrement(&mut self.delay_timer);
    }

    pub fn decrement_sound_timer(&mut self) {
        let decrement = |timer: &mut u8| *timer = (*timer).saturating_sub(1);
        decrement(&mut self.sound_timer);
    }

    pub fn render_display_buffer(&self) {
        for y in 0..SCREEN_HEIGHT as usize{
            for x in 0..SCREEN_WIDTH as usize {
                if self.display_buffer[y][x] {
                    // Draw a white rectangle for each "on" pixel
                    draw_rectangle(
                        (x * WINDOW_SCALE as usize) as f32,     // X position scaled
                        (y * WINDOW_SCALE as usize) as f32,     // Y position scaled
                        WINDOW_SCALE as f32,                    // Width of the rectangle
                        WINDOW_SCALE as f32,                    // Height of the rectangle
                        color::Color::from_hex(PIXEL_ON_COLOR), // Color
                    );
                }
            }
        }
    }

    pub fn handle_input(&mut self) {
        let key_mappings = [
        (input::KeyCode::X, 0),
        (input::KeyCode::Key1, 1),
        (input::KeyCode::Key2, 2),
        (input::KeyCode::Key3, 3),
        (input::KeyCode::Q, 4),
        (input::KeyCode::W, 5),
        (input::KeyCode::E, 6),
        (input::KeyCode::A, 7),
        (input::KeyCode::S, 8),
        (input::KeyCode::D, 9),
        (input::KeyCode::Z, 10),
        (input::KeyCode::C, 11),
        (input::KeyCode::Key4, 12),
        (input::KeyCode::R, 13),
        (input::KeyCode::F, 14),
        (input::KeyCode::V, 15),
    ];

    self.keymap = [false; 16];

    for (key, index) in key_mappings {
        if input::is_key_down(key) {
            self.keymap[index] = true;
        }
    }
    }

    fn load_font(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // a
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // b
            0xF0, 0x80, 0x80, 0x80, 0xF0, // c
            0xE0, 0x90, 0x90, 0x90, 0xE0, // d
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // e
            0xF0, 0x80, 0xF0, 0x80, 0x80, // f
        ];
        for (i, byte) in font.iter().enumerate() {
            self.memory[FONT_POS_START + i] = *byte;
        }
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    fn set_byte(&mut self, addr: u16, val: u8) {
        if (addr as usize) < MEMSIZE {
            self.memory[addr as usize] = val;
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    fn get_addr(&self, addr: u16) -> u8 {
        if (addr as usize) < MEMSIZE {
            self.memory[addr as usize]
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    fn set_pixel(&mut self, x: u8, y: u8, val: bool) -> bool {
        // Clip out of bounds
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return false;
        }
        let pixel = &mut self.display_buffer[y as usize][x as usize];
        let before = *pixel;
        *pixel ^= val;
        if !*pixel && before {
            return true;
        }
        false
    }

    fn draw_sprite_to_display_buffer(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
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

    fn is_key_pressed(&self, target: u8) -> bool {
        if target < 16 {
            return self.keymap[target as usize];
        }
        panic!("is_key_pressed check out of bounds {target}")
    }

    fn clear_screen(&mut self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.display_buffer[i as usize][j as usize] = false;
            }
        }
    }
}
