use std::fmt;

#[derive(Copy, Clone, Debug)]
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

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
//
// EACH OPERATION ADDED HERE WILL NEED TO BE ADDED TO THE MATCH IN THE CORRESPONDING
// fmt FUNCTION. 
pub enum Instruction {
    NOP,
    JMP(Operand),
    PUSH(Register),
    MVI(Register, Operand),
    STA(Operand),
    LXI(Register, Operand), 
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

impl Instruction { 
    fn decode_op(bytes: &[u8]) -> Result<Instruction, ()> {
        let opcode = bytes[0];

        let instruction = match opcode {
            0x00 | 0x10 | 0x20 | 0x30 | 
            0x08 | 0x18 | 0x28 | 0x38  => Instruction::NOP,
            0x01 => Instruction::LXI(Register::B, Operand::D16(Instruction::read_imm16(bytes))),
            0x02 => Instruction::STAX(Register::B),
            0x03 => Instruction::INX(Register::B),
            0x04 => Instruction::INR(Register::B),
            0x05 => Instruction::DCR(Register::B),
            0x06 => Instruction::MVI(Register::B, Operand::D8(Instruction::read_imm8(bytes))),
            0x07 => Instruction::RLC,
            0x09 => Instruction::DAD(Register::B),
            0x0a => Instruction::LDAX(Register::B),
            0x0b => Instruction::DCX(Register::B),
            0x0c => Instruction::INR(Register::C),
            0x0d => Instruction::DCR(Register::C),
            0x0e => Instruction::MVI(Register::C, Operand::D8(Instruction::read_imm8(bytes))),
            0x0f => Instruction::RRC,
            0x11 => Instruction::LXI(Register::D, Operand::D16(Instruction::read_imm16(bytes))),
            0x12 => Instruction::STAX(Register::D),
            0x13 => Instruction::INX(Register::D),
            0x14 => Instruction::INR(Register::D),
            0x15 => Instruction::DCR(Register::D),
            0x16 => Instruction::MVI(Register::D, Operand::D8(Instruction::read_imm8(bytes))),
            0x17 => Instruction::RAL,
            0x19 => Instruction::DAD(Register::D),
            0x1a => Instruction::LDAX(Register::D),
            0x1b => Instruction::DCX(Register::D),
            0x1c => Instruction::INR(Register::E),
            0x1d => Instruction::DCR(Register::E),
            0x1e => Instruction::MVI(Register::E, Operand::D8(Instruction::read_imm8(bytes))),
            0x1f => Instruction::RAR,
            0x21 => Instruction::LXI(Register::H, Operand::D16(Instruction::read_imm16(bytes))),
            0x22 => Instruction::SHLD(Operand::A16(Instruction::read_imm16(bytes))),
            0x23 => Instruction::INX(Register::H),
            0x24 => Instruction::INR(Register::H),
            0x25 => Instruction::DCR(Register::H),
            0x26 => Instruction::MVI(Register::H, Operand::D8(Instruction::read_imm8(bytes))),
            0x27 => Instruction::DAA,
            0x29 => Instruction::DAD(Register::H),
            0x2a => Instruction::LHLD(Operand::A16(Instruction::read_imm16(bytes))),
            0x2b => Instruction::DCX(Register::H),
            0x2c => Instruction::INR(Register::L),
            0x2d => Instruction::DCR(Register::L),
            0x2e => Instruction::MVI(Register::L, Operand::D8(Instruction::read_imm8(bytes))),
            0x2f => Instruction::CMA,
            0x31 => Instruction::LXI(Register::SP, Operand::D16(Instruction::read_imm16(bytes))),
            0x32 => Instruction::STA(Operand::A16(Instruction::read_imm16(bytes))),
            0x33 => Instruction::INX(Register::SP),
            0x34 => Instruction::INR(Register::M),
            0x35 => Instruction::DCR(Register::M),
            0x36 => Instruction::MVI(Register::M, Operand::D8(Instruction::read_imm8(bytes))),
            0x37 => Instruction::STC,
            0x39 => Instruction::DAD(Register::SP),
            0x3a => Instruction::LDA(Operand::A16(Instruction::read_imm16(bytes))),
            0x3b => Instruction::DCX(Register::SP),
            0x3c => Instruction::INR(Register::A),
            0x3d => Instruction::DCR(Register::A),
            0x3e => Instruction::MVI(Register::A, Operand::D8(Instruction::read_imm8(bytes))),
            0x3f => Instruction::CMC,
            0x40 => Instruction::MOV(Register::B, Register::B),
            0x41 => Instruction::MOV(Register::B, Register::C),
            0x42 => Instruction::MOV(Register::B, Register::D),
            0x43 => Instruction::MOV(Register::B, Register::E),
            0x44 => Instruction::MOV(Register::B, Register::H),
            0x45 => Instruction::MOV(Register::B, Register::L),
            0x46 => Instruction::MOV(Register::B, Register::M),
            0x47 => Instruction::MOV(Register::B, Register::A),
            0x48 => Instruction::MOV(Register::C, Register::B),
            0x49 => Instruction::MOV(Register::C, Register::C),
            0x4a => Instruction::MOV(Register::C, Register::D),
            0x4b => Instruction::MOV(Register::C, Register::E),
            0x4c => Instruction::MOV(Register::C, Register::H),
            0x4d => Instruction::MOV(Register::C, Register::L),
            0x4e => Instruction::MOV(Register::C, Register::M),
            0x4f => Instruction::MOV(Register::C, Register::A),
            0x50 => Instruction::MOV(Register::D, Register::B),
            0x51 => Instruction::MOV(Register::D, Register::C),
            0x52 => Instruction::MOV(Register::D, Register::D),
            0x53 => Instruction::MOV(Register::D, Register::E),
            0x54 => Instruction::MOV(Register::D, Register::H),
            0x55 => Instruction::MOV(Register::D, Register::L),
            0x56 => Instruction::MOV(Register::D, Register::M),
            0x57 => Instruction::MOV(Register::D, Register::A),
            0x58 => Instruction::MOV(Register::E, Register::B),
            0x59 => Instruction::MOV(Register::E, Register::C),
            0x5a => Instruction::MOV(Register::E, Register::D),
            0x5b => Instruction::MOV(Register::E, Register::E),
            0x5c => Instruction::MOV(Register::E, Register::H),
            0x5d => Instruction::MOV(Register::E, Register::L),
            0x5e => Instruction::MOV(Register::E, Register::M),
            0x5f => Instruction::MOV(Register::E, Register::A),
            0x60 => Instruction::MOV(Register::H, Register::B),
            0x61 => Instruction::MOV(Register::H, Register::C),
            0x62 => Instruction::MOV(Register::H, Register::D),
            0x63 => Instruction::MOV(Register::H, Register::E),
            0x64 => Instruction::MOV(Register::H, Register::H),
            0x65 => Instruction::MOV(Register::H, Register::L),
            0x66 => Instruction::MOV(Register::H, Register::M),
            0x67 => Instruction::MOV(Register::H, Register::A),
            0x68 => Instruction::MOV(Register::L, Register::B),
            0x69 => Instruction::MOV(Register::L, Register::C),
            0x6a => Instruction::MOV(Register::L, Register::D),
            0x6b => Instruction::MOV(Register::L, Register::E),
            0x6c => Instruction::MOV(Register::L, Register::H),
            0x6d => Instruction::MOV(Register::L, Register::L),
            0x6e => Instruction::MOV(Register::L, Register::M),
            0x6f => Instruction::MOV(Register::L, Register::A),
            0x70 => Instruction::MOV(Register::M, Register::B),
            0x71 => Instruction::MOV(Register::M, Register::C),
            0x72 => Instruction::MOV(Register::M, Register::D),
            0x73 => Instruction::MOV(Register::M, Register::E),
            0x74 => Instruction::MOV(Register::M, Register::H),
            0x75 => Instruction::MOV(Register::M, Register::L),
            0x76 => Instruction::HLT,
            0x77 => Instruction::MOV(Register::M, Register::A),
            0x78 => Instruction::MOV(Register::A, Register::B),
            0x79 => Instruction::MOV(Register::A, Register::C),
            0x7a => Instruction::MOV(Register::A, Register::D),
            0x7b => Instruction::MOV(Register::A, Register::E),
            0x7c => Instruction::MOV(Register::A, Register::H),
            0x7d => Instruction::MOV(Register::A, Register::L),
            0x7e => Instruction::MOV(Register::A, Register::M),
            0x7f => Instruction::MOV(Register::A, Register::A),
            0x80 => Instruction::ADD(Register::B),
            0x81 => Instruction::ADD(Register::C),
            0x82 => Instruction::ADD(Register::D),
            0x83 => Instruction::ADD(Register::E),
            0x84 => Instruction::ADD(Register::H),
            0x85 => Instruction::ADD(Register::L),
            0x86 => Instruction::ADD(Register::M),
            0x87 => Instruction::ADD(Register::A),
            0x88 => Instruction::ADC(Register::B),
            0x89 => Instruction::ADC(Register::C),
            0x8a => Instruction::ADC(Register::D),
            0x8b => Instruction::ADC(Register::E),
            0x8c => Instruction::ADC(Register::H),
            0x8d => Instruction::ADC(Register::L),
            0x8e => Instruction::ADC(Register::M),
            0x8f => Instruction::ADC(Register::A),
            0x90 => Instruction::SUB(Register::B),
            0x91 => Instruction::SUB(Register::C),
            0x92 => Instruction::SUB(Register::D),
            0x93 => Instruction::SUB(Register::E),
            0x94 => Instruction::SUB(Register::H),
            0x95 => Instruction::SUB(Register::L),
            0x96 => Instruction::SUB(Register::M),
            0x97 => Instruction::SUB(Register::A),
            0x98 => Instruction::SBB(Register::B),
            0x99 => Instruction::SBB(Register::C),
            0x9a => Instruction::SBB(Register::D),
            0x9b => Instruction::SBB(Register::E),
            0x9c => Instruction::SBB(Register::H),
            0x9d => Instruction::SBB(Register::L),
            0x9e => Instruction::SBB(Register::M),
            0x9f => Instruction::SBB(Register::A),
            0xa0 => Instruction::ANA(Register::B),
            0xa1 => Instruction::ANA(Register::C),
            0xa2 => Instruction::ANA(Register::D),
            0xa3 => Instruction::ANA(Register::E),
            0xa4 => Instruction::ANA(Register::H),
            0xa5 => Instruction::ANA(Register::L),
            0xa6 => Instruction::ANA(Register::M),
            0xa7 => Instruction::ANA(Register::A),
            0xa8 => Instruction::XRA(Register::B),
            0xa9 => Instruction::XRA(Register::C),
            0xaa => Instruction::XRA(Register::D),
            0xab => Instruction::XRA(Register::E),
            0xac => Instruction::XRA(Register::H),
            0xad => Instruction::XRA(Register::L),
            0xae => Instruction::XRA(Register::M),
            0xaf => Instruction::XRA(Register::A),



            0xc3 => Instruction::JMP(Operand::A16(Instruction::read_imm16(bytes))),
            0xc5 => Instruction::PUSH(Register::B),
            0xd5 => Instruction::PUSH(Register::D),
            0xe5 => Instruction::PUSH(Register::H),
            0xf5 => Instruction::PUSH(Register::PSW),
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
        match *self {
            Instruction::NOP => 1,
            Instruction::JMP(_) => 3,
            Instruction::PUSH(_) => 1,
            Instruction::MVI(_, _) => 2,
            Instruction::STA(_) => 3,
            Instruction::LXI(_,_) => 3,
            Instruction::STAX(_) => 1,
            Instruction::INX(_) => 1,
            Instruction::INR(_) => 1,
            Instruction::DCR(_) => 1,
            Instruction::RLC => 1,
            Instruction::DAD(_) => 1,
            Instruction::LDAX(_) => 1, 
            Instruction::DCX(_) => 1, 
            Instruction::RRC => 1, 
            Instruction::RAL => 1,  
            Instruction::RAR => 1, 
            Instruction::SHLD(_) => 3,
            Instruction::DAA => 1, 
            Instruction::LHLD(_) => 3,
            Instruction::CMA => 1, 
            Instruction::STC => 1,
            Instruction::LDA(_) => 3,
            Instruction::CMC => 1,
            Instruction::MOV(_, _) => 1,
            Instruction::HLT => 1,
            Instruction::ADD(_) => 1,
            Instruction::ANA(_) => 1,
            Instruction::ADC(_) => 1,
            Instruction::SUB(_) => 1,
            Instruction::SBB(_) => 1,
            Instruction::XRA(_) => 1,
            _ => unimplemented!("size for instruction {:#x?} has not yet been implemented", *self)
        }
    }

    pub fn cycles(&self) -> u8 {
        match *self {
            Instruction::NOP => 4,
            Instruction::JMP(_) => 10,
            Instruction::PUSH(_) => 11,
            Instruction::MVI(target, _) => { 
                match target {
                    Register::M => 10,
                    _ => 7
                }
            },
            Instruction::STA(_) => 13,
            Instruction::LXI(_,_) => 10,
            Instruction::STAX(_) => 7,
            Instruction::INX(_) => 5,
            Instruction::INR(target) => {
                match target {
                    Register::M => 10,
                    _ => 5
                }
            },
            Instruction::DCR(target) => {
                match target {
                    Register::M => 10,
                    _ => 5
                }
            },
            Instruction::RLC => 4,
            Instruction::DAD(_) => 10,
            Instruction::LDAX(_) => 7, 
            Instruction::DCX(_) => 5, 
            Instruction::RRC => 4,
            Instruction::RAL => 4, 
            Instruction::RAR => 4,
            Instruction::SHLD(_) => 16,
            Instruction::DAA => 4, 
            Instruction::LHLD(_) => 16,
            Instruction::CMA => 4, 
            Instruction::STC => 4,
            Instruction::LDA(_) => 13,
            Instruction::CMC => 4,
            Instruction::MOV(target, source) => {
                match (target, source) {
                    (Register::M, _) => 7,
                    (_, Register::M) => 7,
                    _ => 5
                }
            },
            Instruction::HLT => 7,
            Instruction::ADD(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            Instruction::ANA(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            Instruction::ADC(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            Instruction::SUB(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            Instruction::SBB(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            Instruction::XRA(target) => {
                match target {
                    Register::M => 7,
                    _ => 4
                }
            },
            _ => unimplemented!("cycles for instruction {:#x?} has not yet been implemented", *self)
        }
    }
}

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Instruction {
        Instruction::decode_op(bytes).unwrap()
    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::D8(val) => write!(f, "{:#x?}", val),
            Operand::A16(val) | Operand::D16(val) => write!(f, "{:#x?}", val),
            _ => write!(f, "Debug printing is not implemented for {:#x?}", self)
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::PUSH(val) => write!(f, "PUSH\t{:#x?}", val),
            Instruction::JMP(val) => write!(f, "JMP\t{:#x?}", val),
            Instruction::MVI(lhs, rhs) => write!(f, "MVI\t{:#x?}, {:#x?}", lhs, rhs),
            Instruction::STA(val) => write!(f, "STA\t{:#x?}", val),
            Instruction::LXI(lhs, rhs) => write!(f, "LXI\t{:#x?}, {:#x?}", lhs, rhs), 
            Instruction::STAX(val) => write!(f, "STAX\t{:#x?}", val),
            Instruction::INX(val) => write!(f, "INX\t{:#x?}", val),
            Instruction::INR(val) => write!(f, "INR\t{:#x?}", val),
            Instruction::DCR(val) => write!(f, "DCR\t{:#x?}", val),
            Instruction::RLC => write!(f, "RLC"),
            Instruction::DAD(val) => write!(f, "DAD\t{:#x?}", val),
            Instruction::LDAX(val) => write!(f, "LDAX\t{:#x?}", val), 
            Instruction::DCX(val) => write!(f, "DCX\t{:#x?}", val), 
            Instruction::RRC => write!(f, "RLC"),
            Instruction::RAL => write!(f, "RLC"), 
            Instruction::RAR => write!(f, "RLC"),
            Instruction::SHLD(val) => write!(f, "SHLD\t{:#x?}", val),
            Instruction::DAA => write!(f, "RLC"), 
            Instruction::LHLD(val) => write!(f, "LHLD\t{:#x?}", val),
            Instruction::CMA => write!(f, "RLC"), 
            Instruction::STC => write!(f, "RLC"),
            Instruction::LDA(val) => write!(f, "LDA\t{:#x?}", val),
            Instruction::CMC => write!(f, "RLC"),
            Instruction::MOV(lhs, rhs) => write!(f, "MOV\t{:#x?}, {:#x?}", lhs, rhs),
            Instruction::HLT => write!(f, "HLT"), 
            Instruction::ADD(val) => write!(f, "ADD\t{:#x?}", val),
            Instruction::ANA(val) => write!(f, "ANA\t{:#x?}", val),
            Instruction::ADC(val) => write!(f, "ADC\t{:#x?}", val),
            Instruction::SUB(val) => write!(f, "SUB\t{:#x?}", val),
            Instruction::SBB(val) => write!(f, "SBB\t{:#x?}", val),
            Instruction::XRA(val) => write!(f, "XRA\t{:#x?}", val),
            _ => unimplemented!("Instruction has not yet been implemented for fmt::Debug")
        }
    }
}
