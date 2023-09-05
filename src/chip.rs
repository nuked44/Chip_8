use std::io::{self, Write};

use rand::Rng;

use crate::{
    config::*,
    inst::{hex_to_inst, Inst},
    screen::Screen,
};

const MEMSIZE: usize = 4096;

pub struct Memory {
    memory: [u8; MEMSIZE],
}

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
            0xa => self.va,
            0xb => self.vb,
            0xc => self.vc,
            0xd => self.vd,
            0xe => self.ve,
            0xf => self.vf,
            _ => panic!("Invalid register get access, reg: v{reg:x}"),
        }
    }

    pub fn set_reg_i(&mut self, val: u16) {
        self.i = val;
    }

    pub fn get_reg_i(&self) -> u16 {
        self.i
    }
}

pub struct Chip {
    pub memory: Memory,
    pub pc: u16,
    pub registers: Register,
    pub stack: [u16; 16],
    pub sp: u8,
    pub screen: Screen,
    pub dt: u8,
    pub st: u8,
}

#[allow(dead_code)]
impl Chip {
    pub fn new() -> Self {
        Chip {
            memory: Memory {
                memory: [0; MEMSIZE],
            },
            pc: 0,
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
            sp: 0,
            screen: Screen::new("Chip-8", SCREEN_WIDTH, SCREEN_HEIGHT),
            dt: 0,
            st: 0,
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
            self.memory.memory[FONT_POS_START + i] = *byte;
        }
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_byte(&mut self, addr: u16, val: u8) {
        if (addr as usize) < MEMSIZE {
            self.memory.memory[addr as usize] = val;
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    pub fn get_addr(&self, addr: u16) -> u8 {
        if (addr as usize) < MEMSIZE {
            self.memory.memory[addr as usize]
        } else {
            panic!("Tried to access memory out of bounds, Memsize: {MEMSIZE}, addr: {addr}");
        }
    }

    pub fn get_keypress(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        for key in self.screen.window.get_keys_pressed(minifb::KeyRepeat::No) {
            match key {
                KEY_0 => ret.push(0x0),
                KEY_1 => ret.push(0x1),
                KEY_2 => ret.push(0x2),
                KEY_3 => ret.push(0x3),
                KEY_4 => ret.push(0x4),
                KEY_5 => ret.push(0x5),
                KEY_6 => ret.push(0x6),
                KEY_7 => ret.push(0x7),
                KEY_8 => ret.push(0x8),
                KEY_9 => ret.push(0x9),
                KEY_A => ret.push(0xA),
                KEY_B => ret.push(0xB),
                KEY_C => ret.push(0xC),
                KEY_D => ret.push(0xD),
                KEY_E => ret.push(0xE),
                KEY_F => ret.push(0xF),
                _ => (),
            }
        }
        ret
    }

    pub fn execute_inst(&mut self) {
        let val: u16 = 0
            | (((self.memory.memory[self.pc as usize] as u16) << 8)
                | self.memory.memory[(self.pc + 1) as usize] as u16);
        let inst: Inst = hex_to_inst(val);
        match inst {
            Inst::Empty => self.pc += 2,
            Inst::Cls => {
                self.screen.clear_screen();
                self.pc += 2;
            }
            Inst::Ret => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            Inst::Jmp { addr } => {
                self.pc = addr;
            }
            Inst::Call { addr } => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
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
                let val = self.registers.get_reg_v(vx);
                self.registers.set_reg_v(vx, val + byte);
                self.pc += 2;
            }
            Inst::LdR { vx, vy } => {
                self.registers.set_reg_v(vx, self.registers.get_reg_v(vy));
                self.pc += 2;
            }
            Inst::OrR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) | self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
                self.pc += 2;
            }
            Inst::AndR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) & self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
                self.pc += 2;
            }
            Inst::XorR { vx, vy } => {
                let val = self.registers.get_reg_v(vx) ^ self.registers.get_reg_v(vy);
                self.registers.set_reg_v(vx, val);
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
            Inst::Shr { vx } => {
                let val = self.registers.get_reg_v(vx);
                self.registers.set_reg_v(0xF, val & 0x1);
                self.registers.set_reg_v(vx, val >> 1);
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
            Inst::Shl { vx } => {
                let val = self.registers.get_reg_v(vx);
                self.registers.set_reg_v(0xF, val & 0x80);
                self.registers.set_reg_v(vx, val << 1);
                self.pc += 2;
            }
            Inst::SnR { vx, vy } => {
                self.pc += 2;
                if self.registers.get_reg_v(vx) != self.registers.get_reg_v(vy) {
                    self.pc += 2;
                }
            }
            Inst::LdI { addr } => {
                self.registers.set_reg_i(addr);
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
                    sprite_buffer
                        .push(self.memory.memory[(self.registers.get_reg_i() + i as u16) as usize]);
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
            }
            Inst::SKp { vx } => {
                let keys = self.get_keypress();
                if !keys.is_empty() {
                    for key in keys {
                        if key == self.registers.get_reg_v(vx) {
                            self.pc += 2;
                            break;
                        }
                    }
                }
                self.pc += 2;
            }
            Inst::SKnp { vx } => {
                let keys: Vec<u8> = self.get_keypress();
                let mut pressed: bool = false;
                if !keys.is_empty() {
                    for key in keys {
                        if key == self.registers.get_reg_v(vx) {
                            pressed = true;
                        }
                    }
                    if pressed {
                        self.pc += 2;
                    }
                }
                self.pc += 2;
            }
            Inst::LdRDt { vx } => {
                self.registers.set_reg_v(vx, self.dt);
                self.pc += 2;
            }
            Inst::LdRKp { vx } => loop {
                let kp = self.get_keypress();
                if !kp.is_empty() {
                    self.registers.set_reg_v(vx, kp[0]);
                    break;
                }
            },
            Inst::LdDtR { vx } => {
                self.dt = self.registers.get_reg_v(vx);
                self.pc += 2;
            }
            Inst::LdStR { vx } => {
                self.st = self.registers.get_reg_v(vx);
                self.pc += 2;
            }
            Inst::AddRI { vx } => {
                let val = self.registers.get_reg_i() + self.registers.get_reg_v(vx) as u16;
                self.registers.set_reg_i(val);
                self.pc += 2;
            }
            Inst::LdIF { vx } => {
                let x = self.registers.get_reg_v(vx);
                if x < 0xF {
                    panic!("Illegal register: v {vx:x}");
                }
                self.registers
                    .set_reg_i((FONT_POS_START + 5 * x as usize) as u16);
            }
            Inst::LdBCDR { vx } => {
                let val = self.registers.get_reg_v(vx);
                self.memory.memory[self.registers.get_reg_i() as usize] = val / 100;
                self.memory.memory[(self.registers.get_reg_i() + 1) as usize] = (val % 100) / 10;
                self.memory.memory[(self.registers.get_reg_i() + 2) as usize] = (val % 100) % 10;
                self.pc += 2;
            }
            Inst::LdIR { vx } => {
                let i = self.registers.get_reg_i();
                for x in 0..vx {
                    self.memory.memory[(i + x as u16) as usize] = self.registers.get_reg_v(x);
                }
                self.pc += 2;
            }
            Inst::LdRI { vx } => {
                let i = self.registers.get_reg_i();
                for x in 0..=vx {
                    self.registers
                        .set_reg_v(x, self.memory.memory[(i + x as u16) as usize]);
                }
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
                print!("{:02x} ", self.memory.memory[x * DBG_LAYOUT_HEIGHT + y]);
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
