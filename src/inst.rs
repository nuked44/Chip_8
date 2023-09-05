#[allow(dead_code)]
pub enum Inst {
    // Does nothing, for syscall
    Empty,
    // Clear the screen
    Cls,
    // Return from subroutine
    Ret,
    // Jmp to addr
    Jmp { addr: u16 },
    // Call subroutine at addr
    Call { addr: u16 },
    // Skip next inst if value in vx == byte
    SV { vx: u8, byte: u8 },
    // Skip next inst if value in vx != byte
    SnV { vx: u8, byte: u8 },
    // Skip if val in vx == val in vy
    SR { vx: u8, vy: u8 },
    // Loads byte into vx
    LdV { vx: u8, byte: u8 },
    // Adds byte to vx
    AddV { vx: u8, byte: u8 },
    // Loads val of vy in vx
    LdR { vx: u8, vy: u8 },
    // Bitwise or of vx and vy, stores result in vx
    OrR { vx: u8, vy: u8 },
    // Bitwise and of vx and vy, stores result in vx
    AndR { vx: u8, vy: u8 },
    // Bitwise xor of vx and vy, stores result in vx
    XorR { vx: u8, vy: u8 },
    // Add vx and vy, result stored in vx, if overflow (vx + vy >= 255) VF set to 1
    AddR { vx: u8, vy: u8 },
    // Subtract vy from vx, result stored in vx, if vx > vy VF set to 1, otherwise 0
    SubR { vx: u8, vy: u8 },
    // Shift vx right, VF set to least significant bit of vx
    Shr { vx: u8 },
    // Subtract vx from vy, result stored in vx, if vy > vx VF set to 1, otherwise 0
    SubnR { vx: u8, vy: u8 },
    // Shift vx left, VF set to most significant bit of vx
    Shl { vx: u8 },
    // Skip if val in vx != val in vy
    SnR { vx: u8, vy: u8 },
    // Load addr into register I
    LdI { addr: u16 },
    // Jumps to addr + V0
    JpV0 { addr: u16 },
    // Moves rnd value (0-255) & byte into vx
    Rnd { vx: u8, byte: u8 },
    // Display n-byte sprite starting at memory location I at (vx, vy), set VF = collision
    Disp { vx: u8, vy: u8, n: u8 },
    // Skip next instruction if key with the value of vx is pressed
    SKp { vx: u8 },
    // Skip next instruction if key with the value of vx is not pressed
    SKnp { vx: u8 },
    // Set vx to delay timer val
    LdRDt { vx: u8 },
    // Wait for a key press, store the value of the key in vx
    LdRKp { vx: u8 },
    // Set delay timer value to vx
    LdDtR { vx: u8 },
    // Set sound timer = vx
    LdStR { vx: u8 },
    // Add vx to I
    AddRI { vx: u8 },
    // Set I = location of font char for val of vx
    LdIF { vx: u8 },
    // Store BCD representation of vx in memory locations pointed to by I, I+1, and I+2
    LdBCDR { vx: u8 },
    // Store registers v0 through vx in memory starting at location I
    LdIR { vx: u8 },
    // Read registers V0 through Vx from memory starting at location I
    LdRI { vx: u8 },
}

#[allow(dead_code)]
pub fn hex_to_inst(val: u16) -> Inst {
    match val & 0xF000 {
        0x0000 => match val {
            0x00E0 => Inst::Cls,
            0x00EE => Inst::Ret,
            _ => Inst::Empty,
        },
        0x1000 => Inst::Jmp { addr: val & 0x0FFF },
        0x2000 => Inst::Call { addr: val & 0x0FFF },
        0x3000 => Inst::SV {
            vx: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x4000 => Inst::SnV {
            vx: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x5000 => Inst::SR {
            vx: ((val & 0x0F00) >> 8) as u8,
            vy: ((val & 0x00F0) >> 4) as u8,
        },
        0x6000 => Inst::LdV {
            vx: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x7000 => Inst::AddV {
            vx: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0x8000 => match val & 0xF00F {
            // 0xF00F has been deliberately chosen for readibility
            0x8000 => Inst::LdR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8001 => Inst::OrR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8002 => Inst::AndR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8003 => Inst::XorR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8004 => Inst::AddR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8005 => Inst::SubR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x8006 => Inst::Shr {
                vx: ((val & 0x0F00) >> 8) as u8,
            },
            0x8007 => Inst::SubnR {
                vx: ((val & 0x0F00) >> 8) as u8,
                vy: ((val & 0x00F0) >> 4) as u8,
            },
            0x800E => Inst::Shl {
                vx: ((val & 0x0F00) >> 8) as u8,
            },
            _ => panic!("Illegal instruction {val}"),
        },
        0x9000 => Inst::SnR {
            vx: ((val & 0x0F00) >> 8) as u8,
            vy: ((val & 0x00F0) >> 4) as u8,
        },
        0xA000 => Inst::LdI { addr: val & 0x0FFF },
        0xB000 => Inst::JpV0 { addr: val & 0x0FFF },
        0xC000 => Inst::Rnd {
            vx: ((val & 0x0F00) >> 8) as u8,
            byte: (val & 0x00FF) as u8,
        },
        0xD000 => Inst::Disp {
            vx: ((val & 0x0F00) >> 8) as u8,
            vy: ((val & 0x00F0) >> 4) as u8,
            n: (val & 0x000F) as u8,
        },
        0xE000 => match val & 0x00FF {
            0x009E => Inst::SKp {
                vx: (val & 0x0F00) as u8,
            },
            0x00A1 => Inst::SKnp {
                vx: (val & 0x0F00) as u8,
            },
            _ => panic!("Illegal instruction {val}"),
        },
        0xF000 => match val & 0x00FF {
            0x0007 => Inst::LdRDt {
                vx: (val & 0x0F00) as u8,
            },
            0x000A => Inst::LdRKp {
                vx: (val & 0x0F00) as u8,
            },
            0x0015 => Inst::LdDtR {
                vx: (val & 0x0F00) as u8,
            },
            0x0018 => Inst::LdStR {
                vx: (val & 0x0F00) as u8,
            },
            0x001E => Inst::AddRI {
                vx: (val & 0x0F00) as u8,
            },
            0x0029 => Inst::LdIF {
                vx: (val & 0x0F00) as u8,
            },
            0x0033 => Inst::LdBCDR {
                vx: (val & 0x0F00) as u8,
            },
            0x0055 => Inst::LdIR {
                vx: (val & 0x0F00) as u8,
            },
            0x0063 => Inst::LdRI {
                vx: (val & 0x0F00) as u8,
            },
            _ => panic!("Illegal instruction {val}"),
        },
        _ => panic!("Illegal instruction {val}"),
    }
}
