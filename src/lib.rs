// TODO need a conversion from u16 to usize, because it is insane to always write "as"
pub mod opcodes;
pub mod types;
use opcodes::{Instruction, Opcode};
pub struct CHIP8 {
    /// Program counter
    pub pc: usize, // should be 16 bits
    /// 16 8 bits registers
    pub registers: [u8; 16],
    /// 4096 bytes memory
    pub memory: [u8; 0x1000],
    /// Stack pointer
    pub sp: usize, // should be 8 bits
    /// 16 slots of 16 bits
    pub stack: [u16; 16], // look like it is not the right official value
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            pc: 0x0,
            registers: [0; 16],
            memory: [0; 0x1000],
            sp: 0x0,
            stack: [0; 16],
        }
    }

    fn read_instruction(&self) -> Instruction {
        let p = self.pc;
        // TODO: maybe can find a more elegant soludtion
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        Instruction::from(op_byte1 << 8 | op_byte2)
    }

    pub fn run(&mut self) {
        loop {
            // should only loop until end of memory
            let instruction = self.read_instruction();
            self.pc += 2;
            match Opcode::from(instruction) {
                Opcode::Opcode0000 => {
                    return; // return instead of ignoring while loop is inefficient
                }
                Opcode::Opcode00EE => self.ret(),
                Opcode::Opcode2NNN(nnn) => self.call(nnn.value()),
                Opcode::Opcode8XY4(x, y) => self.add_xy(x.value(), y.value()),
                Opcode::OpcodeUnknown => todo!("opcode: {:04X}", instruction.raw),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, has_overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if has_overflow {
            self.registers[0xF as usize] = 1;
        } else {
            self.registers[0xF as usize] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        if self.sp > self.stack.len() {
            panic!("Stack overflow");
        }

        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        let addr = self.stack[self.sp];
        self.pc = addr as usize;
    }
}
