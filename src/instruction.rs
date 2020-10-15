use std::fmt;

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
//
// EACH OPERATION ADDED HERE WILL NEED TO BE ADDED TO THE MATCH IN THE CORRESPONDING
// fmt FUNCTION. 
enum Operation {
    NOP,
    JMP(Operand),
    PUSH(Register),
    MVI(Register, Operand),
    STA(Operand)
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    PSW
}

enum Operand {
    A16(u16),
    D8(u8),
    D16(u16)
}

pub struct Instruction {
    op: Operation,
    size: u8,
    cycles: u8
}

impl Instruction {
    fn decode_op(bytes: &[u8]) -> Result<Instruction, ()> {
        let opcode = bytes[0];

        let instruction = match opcode {
            0x00 => Instruction {
                op: Operation::NOP,
                size: 1,
                cycles: 4
            },
            0x32 => Instruction {
                op: Operation::STA(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 13
            },
            0x3e => Instruction {
                op: Operation::MVI(Register::A, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xc3 => Instruction {
                op: Operation::JMP(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xc5 => Instruction {
                op: Operation::PUSH(Register::B),
                size: 1,
                cycles: 11
            },
            0xd5 => Instruction {
                op: Operation::PUSH(Register::D),
                size: 1,
                cycles: 11
            },
            0xe5 => Instruction {
                op: Operation::PUSH(Register::H),
                size: 1,
                cycles: 11
            },
            0xf5 => Instruction {
                op: Operation::PUSH(Register::PSW),
                size: 1,
                cycles: 11
            },
            _ => unimplemented!("instruction {:#x?} has not yet been implemented", opcode)
        };

        Ok(instruction)
    }

    fn read_imm8(bytes: &[u8]) -> u8 {
        u8::from_le_bytes([bytes[1]])
    }

    fn read_imm16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[1], bytes[2]])
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    fn cycles(&self) -> u8 {
        self.cycles
    }
}

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Instruction {
        Instruction::decode_op(bytes).unwrap()
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::NOP => write!(f, "NOP"),
            Operation::PUSH(val) => write!(f, "PUSH {:#x?}", val),
            Operation::JMP(val) => write!(f, "JMP {:#x?}", val),
            Operation::MVI(lhs, rhs) => write!(f, "MVI {:#x?}, {:#x?}", lhs, rhs),
            Operation::STA(val) => write!(f, "STA {:#x?}", val),
            _ => unimplemented!("Operation has not yet been implemented for fmt::Debug")
        }
    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::D8(val) => write!(f, "{:#x?}", val),
            Operand::A16(val) | Operand::D16(val) => write!(f, "{:#x?}", val),
            _ => write!(f, "Debug printing is not implemented for {:#x?}", self)
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self.op);
    }
}

