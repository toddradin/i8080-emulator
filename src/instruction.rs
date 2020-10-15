use std::fmt;

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
//
// EACH OPERATION ADDED HERE WILL NEED TO BE ADDED TO THE MATCH IN THE CORRESPONDING
// fmt FUNCTION. 
enum Operation {
    NOP,
    JMP(Operand),
    PUSH(Register),
    MVI(Register, Operand), //FIX ME
    STA(Operand),
    LXI(Register, Operand), // FIX ME
    STAX(Register),
    INX(Register),
    INR(Register),
    DCR(Register),
    RLC,
    DAD(Register),
    LDAX(Register), 
    DCX(Register), 
    RRC,
    RAL, 
    RAR,
    SHLD(Operand),
    DAA, 
    LHLD(Operand),
    CMA, 
    STC,
    LDA(Operand),
    CMC,
    MOV(Register, Register),
    HLT,
    ADD(Register),
    ANA(Register),
    ADC(Register),
    SUB(Register),
    SBB(Register),
    XRA(Register)
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
    M,
    SP,
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
            0x00 | 0x10 | 0x20 | 0x30 | 
            0x08 | 0x18 | 0x28 | 0x38  => Instruction {
                op: Operation::NOP,
                size: 1,
                cycles: 4
            },
            0x01 => Instruction {
                op: Operation::LXI(Register::B, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x02 => Instruction {
                op: Operation::STAX(Register::B),
                size: 1,
                cycles: 7
            },
            0x03 => Instruction {
                op: Operation::INX(Register::B),
                size: 1,
                cycles: 5
            },
            0x04 => Instruction {
                op: Operation::INR(Register::B),
                size: 1,
                cycles: 5
            },
            0x05 => Instruction {
                op: Operation::DCR(Register::B),
                size: 1,
                cycles: 5
            },
            0x06 => Instruction {
                op: Operation::MVI(Register::B, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x07 => Instruction {
                op: Operation::RLC,
                size: 1,
                cycles: 4
            },
            0x09 => Instruction {
                op: Operation::DAD(Register::B),
                size: 1,
                cycles: 10
            },
            0x0a => Instruction {
                op: Operation::LDAX(Register::B),
                size: 1,
                cycles: 7
            },
            0x0b => Instruction {
                op: Operation::DCX(Register::B),
                size: 1,
                cycles: 5
            },
            0x0c => Instruction {
                op: Operation::INR(Register::C),
                size: 1,
                cycles: 5
            },
            0x0d => Instruction {
                op: Operation::DCR(Register::C),
                size: 1,
                cycles: 5
            },
            0x0e => Instruction {
                op: Operation::MVI(Register::C, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x0f => Instruction {
                op: Operation::RRC,
                size: 1,
                cycles: 4
            },
            0x11 => Instruction {
                op: Operation::LXI(Register::D, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x12 => Instruction {
                op: Operation::STAX(Register::D),
                size: 1,
                cycles: 7
            },
            0x13 => Instruction {
                op: Operation::INX(Register::D),
                size: 1,
                cycles: 5
            },
            0x14 => Instruction {
                op: Operation::INR(Register::D),
                size: 1,
                cycles: 5
            },
            0x15 => Instruction {
                op: Operation::DCR(Register::D),
                size: 1,
                cycles: 5
            },
            0x16 => Instruction {
                op: Operation::MVI(Register::D, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x17 => Instruction {
                op: Operation::RAL,
                size: 1,
                cycles: 4
            },
            0x19 => Instruction {
                op: Operation::DAD(Register::D),
                size: 1,
                cycles: 10
            },
            0x1a => Instruction {
                op: Operation::LDAX(Register::D),
                size: 1,
                cycles: 7
            },
            0x1b => Instruction {
                op: Operation::DCX(Register::D),
                size: 1,
                cycles: 5
            },
            0x1c => Instruction {
                op: Operation::INR(Register::E),
                size: 1,
                cycles: 5
            },
            0x1d => Instruction {
                op: Operation::DCR(Register::E),
                size: 1,
                cycles: 5
            },
            0x1e => Instruction {
                op: Operation::MVI(Register::E, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x1f => Instruction {
                op: Operation::RAR,
                size: 1,
                cycles: 4
            },
            0x21 => Instruction {
                op: Operation::LXI(Register::H, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x22 => Instruction {
                op: Operation::SHLD(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 16
            },
            0x23 => Instruction {
                op: Operation::INX(Register::H),
                size: 1,
                cycles: 5
            },
            0x24 => Instruction {
                op: Operation::INR(Register::H),
                size: 1,
                cycles: 5
            },
            0x25 => Instruction {
                op: Operation::DCR(Register::H),
                size: 1,
                cycles: 5
            },
            0x26 => Instruction {
                op: Operation::MVI(Register::H, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x27 => Instruction {
                op: Operation::DAA,
                size: 1,
                cycles: 4
            },
            0x29 => Instruction {
                op: Operation::DAD(Register::H),
                size: 1,
                cycles: 10
            },
            0x2a => Instruction {
                op: Operation::LHLD(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 16
            },
            0x2b => Instruction {
                op: Operation::DCX(Register::H),
                size: 1,
                cycles: 5
            },
            0x2c => Instruction {
                op: Operation::INR(Register::L),
                size: 1,
                cycles: 5
            },
            0x2d => Instruction {
                op: Operation::DCR(Register::L),
                size: 1,
                cycles: 5
            },
            0x2e => Instruction {
                op: Operation::MVI(Register::L, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x2f => Instruction {
                op: Operation::CMA,
                size: 1,
                cycles: 4
            },
            0x31 => Instruction {
                op: Operation::LXI(Register::SP, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x32 => Instruction {
                op: Operation::STA(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 13
            },
            0x33 => Instruction {
                op: Operation::INX(Register::SP),
                size: 1,
                cycles: 5
            },
            0x34 => Instruction {
                op: Operation::INR(Register::M),
                size: 1,
                cycles: 10
            },
            0x35 => Instruction {
                op: Operation::DCR(Register::M),
                size: 1,
                cycles: 10
            },
            0x36 => Instruction {
                op: Operation::MVI(Register::M, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 10
            },
            0x37 => Instruction {
                op: Operation::STC,
                size: 1,
                cycles: 4
            },
            0x39 => Instruction {
                op: Operation::DAD(Register::SP),
                size: 1,
                cycles: 10
            },
            0x3a => Instruction {
                op: Operation::LDA(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 13
            },
            0x3b => Instruction {
                op: Operation::DCX(Register::SP),
                size: 1,
                cycles: 5
            },
            0x3c => Instruction {
                op: Operation::INR(Register::A),
                size: 1,
                cycles: 5
            },
            0x3d => Instruction {
                op: Operation::DCR(Register::A),
                size: 1,
                cycles: 5
            },
            0x3e => Instruction {
                op: Operation::MVI(Register::A, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x3f => Instruction {
                op: Operation::CMC,
                size: 1,
                cycles: 4
            },
            0x40 =>Instruction {
                op: Operation::MOV(Register::B, Register::B),
                size: 1,
                cycles: 5
            },
            0x41 =>Instruction {
                op: Operation::MOV(Register::B, Register::C),
                size: 1,
                cycles: 5
            },
            0x42 =>Instruction {
                op: Operation::MOV(Register::B, Register::D),
                size: 1,
                cycles: 5
            },
            0x43 =>Instruction {
                op: Operation::MOV(Register::B, Register::E),
                size: 1,
                cycles: 5
            },
            0x44 =>Instruction {
                op: Operation::MOV(Register::B, Register::H),
                size: 1,
                cycles: 5
            },
            0x45 =>Instruction {
                op: Operation::MOV(Register::B, Register::L),
                size: 1,
                cycles: 5
            },
            0x46 =>Instruction {
                op: Operation::MOV(Register::B, Register::M),
                size: 1,
                cycles: 7
            },
            0x47 =>Instruction {
                op: Operation::MOV(Register::B, Register::A),
                size: 1,
                cycles: 5
            },
            0x48 =>Instruction {
                op: Operation::MOV(Register::C, Register::B),
                size: 1,
                cycles: 5
            },
            0x49 =>Instruction {
                op: Operation::MOV(Register::C, Register::C),
                size: 1,
                cycles: 5
            },
            0x4a =>Instruction {
                op: Operation::MOV(Register::C, Register::D),
                size: 1,
                cycles: 5
            },
            0x4b =>Instruction {
                op: Operation::MOV(Register::C, Register::E),
                size: 1,
                cycles: 5
            },
            0x4c =>Instruction {
                op: Operation::MOV(Register::C, Register::H),
                size: 1,
                cycles: 5
            },
            0x4d =>Instruction {
                op: Operation::MOV(Register::C, Register::L),
                size: 1,
                cycles: 5
            },
            0x4e =>Instruction {
                op: Operation::MOV(Register::C, Register::M),
                size: 1,
                cycles: 7
            },
            0x4f =>Instruction {
                op: Operation::MOV(Register::C, Register::A),
                size: 1,
                cycles: 5
            },

            0x50 =>Instruction {
                op: Operation::MOV(Register::D, Register::B),
                size: 1,
                cycles: 5
            },
            0x51 =>Instruction {
                op: Operation::MOV(Register::D, Register::C),
                size: 1,
                cycles: 5
            },
            0x52 =>Instruction {
                op: Operation::MOV(Register::D, Register::D),
                size: 1,
                cycles: 5
            },
            0x53 =>Instruction {
                op: Operation::MOV(Register::D, Register::E),
                size: 1,
                cycles: 5
            },
            0x54 =>Instruction {
                op: Operation::MOV(Register::D, Register::H),
                size: 1,
                cycles: 5
            },
            0x55 =>Instruction {
                op: Operation::MOV(Register::D, Register::L),
                size: 1,
                cycles: 5
            },
            0x56 =>Instruction {
                op: Operation::MOV(Register::D, Register::M),
                size: 1,
                cycles: 7
            },
            0x57 =>Instruction {
                op: Operation::MOV(Register::D, Register::A),
                size: 1,
                cycles: 5
            },
            0x58 =>Instruction {
                op: Operation::MOV(Register::E, Register::B),
                size: 1,
                cycles: 5
            },
            0x59 =>Instruction {
                op: Operation::MOV(Register::E, Register::C),
                size: 1,
                cycles: 5
            },
            0x5a =>Instruction {
                op: Operation::MOV(Register::E, Register::D),
                size: 1,
                cycles: 5
            },
            0x5b =>Instruction {
                op: Operation::MOV(Register::E, Register::E),
                size: 1,
                cycles: 5
            },
            0x5c =>Instruction {
                op: Operation::MOV(Register::E, Register::H),
                size: 1,
                cycles: 5
            },
            0x5d =>Instruction {
                op: Operation::MOV(Register::E, Register::L),
                size: 1,
                cycles: 5
            },
            0x5e =>Instruction {
                op: Operation::MOV(Register::E, Register::M),
                size: 1,
                cycles: 7
            },
            0x5f =>Instruction {
                op: Operation::MOV(Register::E, Register::A),
                size: 1,
                cycles: 5
            },
            0x60 =>Instruction {
                op: Operation::MOV(Register::H, Register::B),
                size: 1,
                cycles: 5
            },
            0x61 =>Instruction {
                op: Operation::MOV(Register::H, Register::C),
                size: 1,
                cycles: 5
            },
            0x62 =>Instruction {
                op: Operation::MOV(Register::H, Register::D),
                size: 1,
                cycles: 5
            },
            0x63 =>Instruction {
                op: Operation::MOV(Register::H, Register::E),
                size: 1,
                cycles: 5
            },
            0x64 =>Instruction {
                op: Operation::MOV(Register::H, Register::H),
                size: 1,
                cycles: 5
            },
            0x65 =>Instruction {
                op: Operation::MOV(Register::H, Register::L),
                size: 1,
                cycles: 5
            },
            0x66 =>Instruction {
                op: Operation::MOV(Register::H, Register::M),
                size: 1,
                cycles: 7
            },
            0x67 =>Instruction {
                op: Operation::MOV(Register::H, Register::A),
                size: 1,
                cycles: 5
            },
            0x68 =>Instruction {
                op: Operation::MOV(Register::L, Register::B),
                size: 1,
                cycles: 5
            },
            0x69 =>Instruction {
                op: Operation::MOV(Register::L, Register::C),
                size: 1,
                cycles: 5
            },
            0x6a =>Instruction {
                op: Operation::MOV(Register::L, Register::D),
                size: 1,
                cycles: 5
            },
            0x6b =>Instruction {
                op: Operation::MOV(Register::L, Register::E),
                size: 1,
                cycles: 5
            },
            0x6c =>Instruction {
                op: Operation::MOV(Register::L, Register::H),
                size: 1,
                cycles: 5
            },
            0x6d =>Instruction {
                op: Operation::MOV(Register::L, Register::L),
                size: 1,
                cycles: 5
            },
            0x6e =>Instruction {
                op: Operation::MOV(Register::L, Register::M),
                size: 1,
                cycles: 7
            },
            0x6f =>Instruction {
                op: Operation::MOV(Register::L, Register::A),
                size: 1,
                cycles: 5
            },
            0x70 =>Instruction {
                op: Operation::MOV(Register::M, Register::B),
                size: 1,
                cycles: 7
            },
            0x71 =>Instruction {
                op: Operation::MOV(Register::M, Register::C),
                size: 1,
                cycles: 7
            },
            0x72 =>Instruction {
                op: Operation::MOV(Register::M, Register::D),
                size: 1,
                cycles: 7
            },
            0x73 =>Instruction {
                op: Operation::MOV(Register::M, Register::E),
                size: 1,
                cycles: 7
            },
            0x74 =>Instruction {
                op: Operation::MOV(Register::M, Register::H),
                size: 1,
                cycles: 7
            },
            0x75 =>Instruction {
                op: Operation::MOV(Register::M, Register::L),
                size: 1,
                cycles: 7
            },
            0x76 =>Instruction {
                op: Operation::HLT,
                size: 1,
                cycles: 7
            },
            0x77 =>Instruction {
                op: Operation::MOV(Register::M, Register::A),
                size: 1,
                cycles: 7
            },
            0x78 =>Instruction {
                op: Operation::MOV(Register::A, Register::B),
                size: 1,
                cycles: 5
            },
            0x79 =>Instruction {
                op: Operation::MOV(Register::A, Register::C),
                size: 1,
                cycles: 5
            },
            0x7a =>Instruction {
                op: Operation::MOV(Register::A, Register::D),
                size: 1,
                cycles: 5
            },
            0x7b =>Instruction {
                op: Operation::MOV(Register::A, Register::E),
                size: 1,
                cycles: 5
            },
            0x7c =>Instruction {
                op: Operation::MOV(Register::A, Register::H),
                size: 1,
                cycles: 5
            },
            0x7d =>Instruction {
                op: Operation::MOV(Register::A, Register::L),
                size: 1,
                cycles: 5
            },
            0x7e =>Instruction {
                op: Operation::MOV(Register::A, Register::M),
                size: 1,
                cycles: 7
            },
            0x7f =>Instruction {
                op: Operation::MOV(Register::A, Register::A),
                size: 1,
                cycles: 5
            },
            0x80 =>Instruction {
                op: Operation::ADD(Register::B),
                size: 1,
                cycles: 4
            },
            0x81 =>Instruction {
                op: Operation::ADD(Register::C),
                size: 1,
                cycles: 4
            },
            0x82 =>Instruction {
                op: Operation::ADD(Register::D),
                size: 1,
                cycles: 4
            },
            0x83 =>Instruction {
                op: Operation::ADD(Register::E),
                size: 1,
                cycles: 4
            },
            0x84 =>Instruction {
                op: Operation::ADD(Register::H),
                size: 1,
                cycles: 4
            },
            0x85 =>Instruction {
                op: Operation::ADD(Register::L),
                size: 1,
                cycles: 4
            },
            0x86 =>Instruction {
                op: Operation::ADD(Register::M),
                size: 1,
                cycles: 7
            },
            0x87 =>Instruction {
                op: Operation::ADD(Register::A),
                size: 1,
                cycles: 4
            },
            0x88 =>Instruction {
                op: Operation::ADC(Register::B),
                size: 1,
                cycles: 4
            },
            0x89 =>Instruction {
                op: Operation::ADC(Register::C),
                size: 1,
                cycles: 4
            },
            0x8a =>Instruction {
                op: Operation::ADC(Register::D),
                size: 1,
                cycles: 4
            },
            0x8b =>Instruction {
                op: Operation::ADC(Register::E),
                size: 1,
                cycles: 4
            },
            0x8c =>Instruction {
                op: Operation::ADC(Register::H),
                size: 1,
                cycles: 4
            },
            0x8d =>Instruction {
                op: Operation::ADC(Register::L),
                size: 1,
                cycles: 4
            },
            0x8e =>Instruction {
                op: Operation::ADC(Register::M),
                size: 1,
                cycles: 7
            },
            0x8f =>Instruction {
                op: Operation::ADC(Register::A),
                size: 1,
                cycles: 4
            },
            0x90 =>Instruction {
                op: Operation::SUB(Register::B),
                size: 1,
                cycles: 4
            },
            0x91 =>Instruction {
                op: Operation::SUB(Register::C),
                size: 1,
                cycles: 4
            },
            0x92 =>Instruction {
                op: Operation::SUB(Register::D),
                size: 1,
                cycles: 4
            },
            0x93 =>Instruction {
                op: Operation::SUB(Register::E),
                size: 1,
                cycles: 4
            },
            0x94 =>Instruction {
                op: Operation::SUB(Register::H),
                size: 1,
                cycles: 4
            },
            0x95 =>Instruction {
                op: Operation::SUB(Register::L),
                size: 1,
                cycles: 4
            },
            0x96 =>Instruction {
                op: Operation::SUB(Register::M),
                size: 1,
                cycles: 7
            },
            0x97 =>Instruction {
                op: Operation::SUB(Register::A),
                size: 1,
                cycles: 4
            },
            0x98 =>Instruction {
                op: Operation::SBB(Register::B),
                size: 1,
                cycles: 4
            },
            0x99 =>Instruction {
                op: Operation::SBB(Register::C),
                size: 1,
                cycles: 4
            },
            0x9a =>Instruction {
                op: Operation::SBB(Register::D),
                size: 1,
                cycles: 4
            },
            0x9b =>Instruction {
                op: Operation::SBB(Register::E),
                size: 1,
                cycles: 4
            },
            0x9c =>Instruction {
                op: Operation::SBB(Register::H),
                size: 1,
                cycles: 4
            },
            0x9d =>Instruction {
                op: Operation::SBB(Register::L),
                size: 1,
                cycles: 4
            },
            0x9e =>Instruction {
                op: Operation::SBB(Register::M),
                size: 1,
                cycles: 7
            },
            0x9f =>Instruction {
                op: Operation::SBB(Register::A),
                size: 1,
                cycles: 4
            },
            0xa0 =>Instruction {
                op: Operation::ANA(Register::B),
                size: 1,
                cycles: 4
            },
            0xa1 =>Instruction {
                op: Operation::ANA(Register::C),
                size: 1,
                cycles: 4
            },
            0xa2 =>Instruction {
                op: Operation::ANA(Register::D),
                size: 1,
                cycles: 4
            },
            0xa3 =>Instruction {
                op: Operation::ANA(Register::E),
                size: 1,
                cycles: 4
            },
            0xa4 =>Instruction {
                op: Operation::ANA(Register::H),
                size: 1,
                cycles: 4
            },
            0xa5 =>Instruction {
                op: Operation::ANA(Register::L),
                size: 1,
                cycles: 4
            },
            0xa6 =>Instruction {
                op: Operation::ANA(Register::M),
                size: 1,
                cycles: 7
            },
            0xa7 =>Instruction {
                op: Operation::ANA(Register::A),
                size: 1,
                cycles: 4
            },
            0xa8 =>Instruction {
                op: Operation::XRA(Register::B),
                size: 1,
                cycles: 4
            },
            0xa9 =>Instruction {
                op: Operation::ANA(Register::C),
                size: 1,
                cycles: 4
            },
            0xaa =>Instruction {
                op: Operation::ANA(Register::D),
                size: 1,
                cycles: 4
            },
            0xab =>Instruction {
                op: Operation::ANA(Register::E),
                size: 1,
                cycles: 4
            },
            0xac =>Instruction {
                op: Operation::ANA(Register::H),
                size: 1,
                cycles: 4
            },
            0xad =>Instruction {
                op: Operation::ANA(Register::L),
                size: 1,
                cycles: 4
            },
            0xae =>Instruction {
                op: Operation::ANA(Register::M),
                size: 1,
                cycles: 7
            },
            0xaf =>Instruction {
                op: Operation::ANA(Register::A),
                size: 1,
                cycles: 4
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
            Operation::PUSH(val) => write!(f, "PUSH\t{:#x?}", val),
            Operation::JMP(val) => write!(f, "JMP\t{:#x?}", val),
            Operation::MVI(lhs, rhs) => write!(f, "MVI\t{:#x?}, {:#x?}", lhs, rhs),
            Operation::STA(val) => write!(f, "STA\t{:#x?}", val),
            Operation::LXI(lhs, rhs) => write!(f, "LXI\t{:#x?}, {:#x?}", lhs, rhs), 
            Operation::STAX(val) => write!(f, "STAX\t{:#x?}", val),
            Operation::INX(val) => write!(f, "INX\t{:#x?}", val),
            Operation::INR(val) => write!(f, "INR\t{:#x?}", val),
            Operation::DCR(val) => write!(f, "DCR\t{:#x?}", val),
            Operation::RLC => write!(f, "RLC"),
            Operation::DAD(val) => write!(f, "DAD\t{:#x?}", val),
            Operation::LDAX(val) => write!(f, "LDAX\t{:#x?}", val), 
            Operation::DCX(val) => write!(f, "DCX\t{:#x?}", val), 
            Operation::RRC => write!(f, "RLC"),
            Operation::RAL => write!(f, "RLC"), 
            Operation::RAR => write!(f, "RLC"),
            Operation::SHLD(val) => write!(f, "SHLD\t{:#x?}", val),
            Operation::DAA => write!(f, "RLC"), 
            Operation::LHLD(val) => write!(f, "LHLD\t{:#x?}", val),
            Operation::CMA => write!(f, "RLC"), 
            Operation::STC => write!(f, "RLC"),
            Operation::LDA(val) => write!(f, "LDA\t{:#x?}", val),
            Operation::CMC => write!(f, "RLC"),
            Operation::MOV(lhs, rhs) => write!(f, "MOV\t{:#x?}, {:#x?}", lhs, rhs),
            Operation::HLT => write!(f, "HLT"), 
            Operation::ADD(val) => write!(f, "ADD\t{:#x?}", val),
            Operation::ANA(val) => write!(f, "ANA\t{:#x?}", val),
            Operation::ADC(val) => write!(f, "ADC\t{:#x?}", val),
            Operation::SUB(val) => write!(f, "SUB\t{:#x?}", val),
            Operation::SBB(val) => write!(f, "SBB\t{:#x?}", val),
            Operation::XRA(val) => write!(f, "XRA\t{:#x?}", val),
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

