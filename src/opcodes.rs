use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub raw: u16,
}

impl Instruction {
    pub fn new(raw: u16) -> Instruction {
        Instruction { raw }
    }

    pub fn opcode(&self) -> Opcode {
        Opcode::from(*self)
    }

    pub fn extract_nibbles(&self) -> (Nibble, Nibble, Nibble, Nibble) {
        let nibble1 = (((self.raw & 0xF000) >> 12) as u8).into();
        let nibble2 = (((self.raw & 0x0F00) >> 8) as u8).into();
        let nibble3 = (((self.raw & 0x00F0) >> 4) as u8).into();
        let nibble4 = (((self.raw & 0x000F) >> 0) as u8).into();
        (nibble1, nibble2, nibble3, nibble4)
    }
    pub fn extract_immediate_value(&self) -> ImmediateValue {
        ((self.raw & 0xFF) as u8).into()
    }

    pub fn extract_address(&self) -> Address {
        ((self.raw & 0xFFF) as u16).into()
    }
}

impl From<u16> for Instruction {
    fn from(raw: u16) -> Self {
        Instruction::new(raw)
    }
}

pub enum Opcode {
    Opcode8XY4(Register, Register),
    Opcode00EE,
    Opcode2NNN(Address),
    Opcode0000,
    OpcodeUnknown,
}

use Opcode::*;
impl From<Instruction> for Opcode {
    fn from(instruction: Instruction) -> Opcode {
        let (c, x, y, d) = instruction.extract_nibbles();
        let nn = instruction.extract_immediate_value();
        let nnn = instruction.extract_address();
        match (c, x, y, d) {
            (Nibble(0x0), Nibble(0x0), Nibble(0x0), Nibble(0x0)) => Opcode0000,
            (Nibble(0x8), _, _, Nibble(0x4)) => Opcode8XY4(x, y),
            (Nibble(0x0), Nibble(0x0), Nibble(0xE), Nibble(0xE)) => Opcode00EE,
            (Nibble(0x2), _, _, _) => Opcode2NNN(nnn),
            _ => panic!("DA FUCK {:04X}", instruction.raw),
        }
    }
}
