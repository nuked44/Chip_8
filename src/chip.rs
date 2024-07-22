use std::io::{self, Write};

use rand::Rng;

use crate::{
    config::*,
    inst::{hex_to_inst, Inst},
    screen::Screen,
};

#[allow(dead_code)]
pub struct Register {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    i: u16,
}

#[allow(dead_code)]
impl Register {
    pub fn set_reg_v(&mut self, reg: u8, val: u8) {
        match reg {
            0x0 => self.v0 = val,
            0x1 => self.v1 = val,
            0x2 => self.v2 = val,
            0x3 => self.v3 = val,
            0x4 => self.v4 = val,
            0x5 => self.v5 = val,
            0x6 => self.v6 = val,
            0x7 => self.v7 = val,
            0x8 => self.v8 = val,
            0x9 => self.v9 = val,
            0xa => self.va = val,
            0xb => self.vb = val,
            0xc => self.vc = val,
            0xd => self.vd = val,
            0xe => self.ve = val,
            0xf => self.vf = val,
            _ => panic!("Invalid register set access, reg: v{reg:x}"),
        };
    }

    pub fn get_reg_v(&self, reg: u8) -> u8 {
        match reg {
            0x0 => self.v0,
            0x1 => self.v1,
            0x2 => self.v2,
            0x3 => self.v3,
            0x4 => self.v4,
            0x5 => self.v5,
            0x6 => self.v6,
            0x7 => self.v7,
            0x8 => self.v8,
            0x9 => self.v9,
            0xA => self.va,
            0xB => self.vb,
            0xC => self.vc,
            0xD => self.vd,
            0xE => self.ve,
            0xF => self.vf,
            _ => panic!("Invalid register get access, reg: v {reg:02x}"),
        }
    }
}

pub struct Chip {
    pub memory: [u8; MEMSIZE],
    pub pc: u16,
    pub registers: Register,
    pub stack: [u16; 16],
    pub stackpointer: u8,
    pub screen: Screen,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keyboard: Option<u8>,
    release_key_wait: Option<u8>,
}

#[allow(dead_code)]
impl Chip {
    pub fn new(prog_counter: u16, screen: Screen) -> Self {
        Chip {
            memory: [0; MEMSIZE],
            pc: prog_counter,
            registers: Register {
                v0: 0,
                v1: 0,
                v2: 0,
                v3: 0,
                v4: 0,
                v5: 0,
                v6: 0,
                v7: 0,
                v8: 0,
                v9: 0,
                va: 0,
                vb: 0,
                vc: 0,
                vd: 0,
                ve: 0,
                vf: 0,
                i: 0,
            },
            stack: [0; 16],
            stackpointer: 0,
            screen,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: None,
            release_key_wait: None,
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

    pub fn load_prog(&mut self, prog: Vec<u8>) {
        for (i, inst_part) in prog.iter().enumerate() {
            self.memory[PROG_POS_START as usize + i] = *inst_part;
        }
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_byte(&mut self, addr: u16, val: u8) {
        if (addr as usize) < MEMSIZE {
            self.memory[addr as usize] = val;
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    pub fn get_addr(&self, addr: u16) -> u8 {
        if (addr as usize) < MEMSIZE {
            self.memory[addr as usize]
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    pub fn get_key(&self, key: u8) -> Option<bool> {
        match key {
            0x0 => Some(self.screen.window.is_key_down(KEY_0)),
            0x1 => Some(self.screen.window.is_key_down(KEY_1)),
            0x2 => Some(self.screen.window.is_key_down(KEY_2)),
            0x3 => Some(self.screen.window.is_key_down(KEY_3)),
            0x4 => Some(self.screen.window.is_key_down(KEY_4)),
            0x5 => Some(self.screen.window.is_key_down(KEY_5)),
            0x6 => Some(self.screen.window.is_key_down(KEY_6)),
            0x7 => Some(self.screen.window.is_key_down(KEY_7)),
            0x8 => Some(self.screen.window.is_key_down(KEY_8)),
            0x9 => Some(self.screen.window.is_key_down(KEY_9)),
            0xA => Some(self.screen.window.is_key_down(KEY_A)),
            0xB => Some(self.screen.window.is_key_down(KEY_B)),
            0xC => Some(self.screen.window.is_key_down(KEY_C)),
            0xD => Some(self.screen.window.is_key_down(KEY_D)),
            0xE => Some(self.screen.window.is_key_down(KEY_E)),
            0xF => Some(self.screen.window.is_key_down(KEY_F)),
            _ => None,
        }
    }

    pub fn key_to_u8(&self, key: Option<minifb::Key>) -> Option<u8> {
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

    pub fn execute_inst(&mut self) {
        let val: u16 = 0
            | (((self.memory[self.pc as usize] as u16) << 8)
                | self.memory[(self.pc + 1) as usize] as u16);
        let inst: Inst = hex_to_inst(val);
        match inst {
            Inst::Empty => self.pc += 2,
            Inst::Cls => {
                self.screen.clear_screen();
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
                if self.registers.get_reg_v(vx) == byte {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SnV { vx, byte } => {
                if self.registers.get_reg_v(vx) != byte {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SR { vx, vy } => {
                if self.registers.get_reg_v(vx) == self.registers.get_reg_v(vy) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::LdV { vx, byte } => {
                self.registers.set_reg_v(vx, byte);
                self.pc += 2;
            }
            Inst::AddV { vx, byte } => {
                let (res, _) = self.registers.get_reg_v(vx).overflowing_add(byte);
                self.registers.set_reg_v(vx, res);
                self.pc += 2;
            }
            Inst::LdR { vx, vy } => {
                self.registers.set_reg_v(vx, self.registers.get_reg_v(vy));
                self.pc += 2;
            }
            Inst::OrR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) | self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
                self.registers.set_reg_v(0xF, 0);
                self.pc += 2;
            }
            Inst::AndR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) & self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
                self.registers.set_reg_v(0xF, 0);
                self.pc += 2;
            }
            Inst::XorR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) ^ self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
                self.registers.set_reg_v(0xF, 0);
                self.pc += 2;
            }
            Inst::AddR { vx, vy } => {
                let (res, vf) = self
                    .registers
                    .get_reg_v(vx)
                    .overflowing_add(self.registers.get_reg_v(vy));
                self.registers.set_reg_v(vx, res);
                self.registers.set_reg_v(0xF, vf as u8);
                self.pc += 2;
            }
            Inst::SubR { vx, vy } => {
                let (res, vf) = self
                    .registers
                    .get_reg_v(vx)
                    .overflowing_sub(self.registers.get_reg_v(vy));
                self.registers.set_reg_v(vx, res);
                self.registers.set_reg_v(0xF, !vf as u8);
                self.pc += 2;
            }
            Inst::Shr { vx, vy } => {
                let val = self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val >> 1);
                self.registers.set_reg_v(0xF, val & 0x1);
                self.pc += 2;
            }
            Inst::SubnR { vx, vy } => {
                let (res, vf) = self
                    .registers
                    .get_reg_v(vy)
                    .overflowing_sub(self.registers.get_reg_v(vx));
                self.registers.set_reg_v(vx, res);
                self.registers.set_reg_v(0xF, !vf as u8);
                self.pc += 2;
            }
            Inst::Shl { vx, vy } => {
                let val = self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val << 1);
                self.registers.set_reg_v(0xF, (val >> 7) & 0x1);
                self.pc += 2;
            }
            Inst::SnR { vx, vy } => {
                self.pc += 2;
                if self.registers.get_reg_v(vx) != self.registers.get_reg_v(vy) {
                    self.pc += 2;
                }
            }
            Inst::LdI { addr } => {
                self.registers.i = addr;
                self.pc += 2;
            }
            Inst::JpV0 { addr } => {
                self.pc = addr + self.registers.get_reg_v(0x0) as u16;
            }
            Inst::Rnd { vx, byte } => {
                let val = (rand::thread_rng().gen_range(0..=255) as u8) & byte;
                self.registers.set_reg_v(vx, val);
                self.pc += 2;
            }
            Inst::Disp { vx, vy, n } => {
                let mut sprite_buffer: Vec<u8> = Vec::new();
                for i in 0..n {
                    sprite_buffer.push(self.memory[(self.registers.i + i as u16) as usize]);
                }
                if self.screen.draw_sprite(
                    self.registers.get_reg_v(vx),
                    self.registers.get_reg_v(vy),
                    sprite_buffer,
                ) {
                    self.registers.set_reg_v(0xF, 1);
                } else {
                    self.registers.set_reg_v(0xF, 0);
                }
                self.pc += 2;
            }
            Inst::SKp { vx } => {
                let target = self.registers.get_reg_v(vx);
                if self.get_key(target).unwrap() {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SKnp { vx } => {
                let target = self.registers.get_reg_v(vx);
                if !self.get_key(target).unwrap() {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::LdRDt { vx } => {
                self.registers.set_reg_v(vx, self.delay_timer);
                self.pc += 2;
            }
            Inst::LdRKp { vx } => {
                if let Some(key) = self.release_key_wait {
                    if !self.get_key(key).unwrap() {
                        self.release_key_wait = None;
                        self.pc += 2;
                    }
                } else {
                    if let Some(key) = self.keyboard {
                        self.registers.set_reg_v(vx, key);
                        self.release_key_wait = Some(key);
                    }
                }
            }
            Inst::LdDtR { vx } => {
                self.delay_timer = self.registers.get_reg_v(vx);
                self.pc += 2;
            }
            Inst::LdStR { vx } => {
                self.sound_timer = self.registers.get_reg_v(vx);
                self.pc += 2;
            }
            Inst::AddRI { vx } => {
                let val = self.registers.i + self.registers.get_reg_v(vx) as u16;
                self.registers.i = val;
                self.pc += 2;
            }
            Inst::LdIF { vx } => {
                let x = self.registers.get_reg_v(vx);
                self.registers.i = (FONT_POS_START + 5 * x as usize) as u16;
                self.pc += 2;
            }
            Inst::LdBCDR { vx } => {
                let val = self.registers.get_reg_v(vx);
                self.memory[self.registers.i as usize] = val / 100;
                self.memory[(self.registers.i + 1) as usize] = (val % 100) / 10;
                self.memory[(self.registers.i + 2) as usize] = (val % 100) % 10;
                self.pc += 2;
            }
            Inst::LdIR { vx } => {
                let i = self.registers.i;
                for x in 0..=vx {
                    self.memory[(i + x as u16) as usize] = self.registers.get_reg_v(x);
                }
                self.registers.i = i + vx as u16 + 1;
                self.pc += 2;
            }
            Inst::LdRI { vx } => {
                let i = self.registers.i;
                for x in 0..=vx {
                    self.registers
                        .set_reg_v(x, self.memory[(i + x as u16) as usize]);
                }
                self.registers.i = i + vx as u16 + 1;
                self.pc += 2;
            }
        }
    }
}

// ==========  DBG functions ========== //

const DBG_LAYOUT_WIDTH: usize = 1 << 4;
const DBG_LAYOUT_HEIGHT: usize = 1 << 8; // rhs has to equal 12 (e.g. 8 and 4)

#[allow(dead_code)]
impl Chip {
    fn print_mem(&self) {
        for x in 0..DBG_LAYOUT_HEIGHT {
            print!(
                "{:04x}-{:04x}: ",
                x * DBG_LAYOUT_WIDTH,
                x * DBG_LAYOUT_WIDTH + DBG_LAYOUT_WIDTH - 1
            );
            for y in 0..DBG_LAYOUT_WIDTH {
                print!("{:02x} ", self.memory[x * DBG_LAYOUT_HEIGHT + y]);
            }
            println!();
        }
        io::stdout().flush().expect("unable to flush memory state");
    }

    fn print_pc(&self) {
        println!("PC: {}", self.pc);
    }

    pub fn print_chip(&self) {
        self.print_mem();
        self.print_pc();
    }

    pub fn print_info(msg: &str) {
        println!("[ INFO ]: {msg}");
    }

    pub fn print_fatal(msg: &str) {
        println!("[ FATAL ]: {msg}");
    }
}
