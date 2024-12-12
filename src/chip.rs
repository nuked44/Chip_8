use std::time::{Duration, Instant};

use rand::Rng;

use crate::{
    config::*,
    inst::{hex_to_inst, Inst},
    screen::Interface,
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

pub struct Chip<T>
where
    T: Interface,
{
    pub running: bool,
    pub memory: [u8; MEMSIZE],
    pub pc: u16,
    pub registers: Register,
    pub stack: [u16; 16],
    pub stackpointer: u8,
    pub interface: T,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keyboard: Option<u8>,
    release_key_wait: Option<u8>,
}

#[allow(dead_code)]
impl<T: Interface> Chip<T> {
    pub fn new(prog_counter: u16, interface: T) -> Self {
        Chip {
            running: false,
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
            interface,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: None,
            release_key_wait: None,
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        let target_frame_time = Duration::from_secs_f64(1f64 / SCREEN_REFRESH_RATE as f64);
        let mut last_frame = Instant::now();

        let decrement = |timer: &mut u8| *timer = (*timer).saturating_sub(1);

        while self.running && !self.interface.get_close_window() {
            // Execute next instructions for frame
            for _ in 0..(INSTRUCTION_FREQUENCY / SCREEN_REFRESH_RATE) {
                self.execute_inst();
            }

            // Update Screen
            self.interface.update_screen();

            // Update Sound and Delay timer
            decrement(&mut self.delay_timer);
            decrement(&mut self.sound_timer);

            let now_frame = Instant::now();
            if (now_frame - last_frame) < target_frame_time {
                std::thread::sleep(target_frame_time - (now_frame - last_frame));
            }
            last_frame = now_frame;
        }
    }

    pub fn init_interface(&self) {
        self.interface.init();
    }

    pub fn stop_interface(&self) {
        self.interface.stop();
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
                self.interface.clear_screen();
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
                if self.interface.draw_sprite(
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
                if self.interface.get_key(target) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Inst::SKnp { vx } => {
                let target = self.registers.get_reg_v(vx);
                if !self.interface.get_key(target) {
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
                    if !self.interface.get_key(key) {
                        self.release_key_wait = None;
                        self.pc += 2;
                    }
                } else if let Some(key) = self.keyboard {
                    self.registers.set_reg_v(vx, key);
                    self.release_key_wait = Some(key);
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
}
