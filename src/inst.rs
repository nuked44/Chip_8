pub enum Inst {
    // Does nothing, for syscall
    EmptyInst,
    // Clear the screen
    Cls,
    // Return from subroutine
    Ret,
    // Jmp to addr
    Jmp { addr: u16 },
    // Call subroutine at addr
    Call { addr: u16 },
    // Skip next inst if value in reg == byte
    CVSkip { reg: u8, byte: u8 },
    // Skip next inst if value in reg != byte
    CNSkip { reg: u8, byte: u8 },
    // Skip if val in reg1 == val in reg2
    CRSkip { reg1: u8, reg2: u8 },
    // Loads byte into reg
    LdV { reg: u8, byte: u8 },
    // Adds byte to reg
    AddV { reg: u8, byte: u8 },
    // Loads val of reg2 in reg1
    LdR { reg1: u8, reg2: u8 },
    // Bitwise or of reg1 and reg2, stores result in reg1
    OrR { reg1: u8, reg2: u8 },
    // Bitwise and of reg1 and reg2, stores result in reg1
    AndR { reg1: u8, reg2: u8 },
    // Bitwise xor of reg1 and reg2, stores result in reg1
    XorR { reg1: u8, reg2: u8 },
    // Add reg1 and reg2, result stored in reg1, if overflow (reg1 + reg2 >= 255) VF set to 1
    AddR { reg1: u8, reg2: u8 },
}

pub fn hex_to_inst(val: u16) -> Inst {
    match val & 0xF000 {
        0x0000 => match val {
            0x00E0 => Inst::Cls,
            0x00EE => Inst::Ret,
            _ => Inst::EmptyInst,
        },
        0x1000 => Inst::Jmp { addr: val & 0x0FFF },
        0x2000 => Inst::Call { addr: val & 0x0FFF },
        0x3000 => Inst::CVSkip {
            reg: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x4000 => Inst::CNSkip {
            reg: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x5000 => Inst::CRSkip {
            reg1: ((val & 0x0F00) >> 8) as u8,
            reg2: ((val & 0x00F0) >> 4) as u8,
        },
        0x6000 => Inst::LdV {
            reg: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x7000 => Inst::AddV {
            reg: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x8000 => match val & 0xF00F {
            // 0xF00F has been deliberately chosen for readibility
            0x8000 => Inst::LdR {
                reg1: ((val & 0x0F00) >> 8) as u8,
                reg2: ((val & 0x00F0) >> 4) as u8,
            },
            0x8001 => Inst::OrR {
                reg1: ((val & 0x0F00) >> 8) as u8,
                reg2: ((val & 0x00F0) >> 4) as u8,
            },
            0x8002 => Inst::AndR {
                reg1: ((val & 0x0F00) >> 8) as u8,
                reg2: ((val & 0x00F0) >> 4) as u8,
            },
            0x8003 => Inst::XorR {
                reg1: ((val & 0x0F00) >> 8) as u8,
                reg2: ((val & 0x00F0) >> 4) as u8,
            },
            0x8004 => Inst::AddR {
                reg1: ((val & 0x0F00) >> 8) as u8,
                reg2: ((val & 0x00F0) >> 4) as u8,
            },
            // TODO: inst 0x8005 ff.
            _ => panic!("Illegal instruction {val}"),
        },
        _ => panic!("Illegal instruction {val}"),
    }
}
