use std::io::{self, Write};

use crate::screen::{self, Screen};

const MEMSIZE: usize = 4096;

pub struct Memory {
    memory: [u8; MEMSIZE],
}

pub struct Registers {
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

pub struct Chip {
    pub memory: Memory,
    pub pc: u16,
    pub registers: Registers,
    pub sp: u8,
    pub stack: [u16; 16],
    pub screen: Screen,
}

impl Chip {
    pub fn new() -> Self {
        return Chip {
            memory: Memory {
                memory: [0; MEMSIZE],
            },
            pc: 0,
            registers: Registers {
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
            sp: 0,
            stack: [0; 16],
            screen: Screen::new("Chip-8", screen::SCREEN_WIDTH, screen::SCREEN_HEIGHT),
        };
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_byte(&mut self, byte: u16, val: u8) {
        if (byte as usize) < MEMSIZE {
            self.memory.memory[byte as usize] = val;
        }
    }

    pub fn get_byte(&self, byte: u16) -> u8 {
        if (byte as usize) < MEMSIZE {
            return self.memory.memory[byte as usize];
        } else {
            return 0;
        }
    }
}

// ==========  DBG functions ========== //

const DBG_LAYOUT_WIDTH: usize = 1 << 4;
const DBG_LAYOUT_HEIGHT: usize = 1 << 8; // rhs has to equal 12 (e.g. 8 and 4)

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
            print!("\n");
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
}
